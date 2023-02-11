//! Library for `mausam`
// $ RUST_BACKTRACE=1 mausam

mod temperature;

use std::{env, num::ParseFloatError, path::PathBuf};

use anyhow::{anyhow, Context};
use clap::Parser;
use dotenv::dotenv;
use lazy_static::lazy_static;
use notify_rust::{Hint, Notification};
use reqwest::{Client, Response};
use rust_decimal::Decimal;

use crate::{cli::Cli, models::OpenWeatherData};

// Define the URL and the client as lazily loaded statics
lazy_static! {
    pub static ref IP_API_URL: &'static str = "http://ip-api.com/json";
    pub static ref CLIENT: Client = Client::new();
}

// HACK: Can use RUST_PACKAGE name env?
pub const APP_NAME: &str = "mausam";

pub async fn run() -> anyhow::Result<OpenWeatherData> {
    dotenv().ok();

    let location = ipapi::get_ip_api_location().await?;
    let city = location.city;
    // println!("Your current city is: {city}.");

    let mut args = Cli::parse();
    let place = args.place.get_or_insert(city);
    if place.is_empty() {
        panic!("{:#?}", anyhow!("`{place}`").context("Empty string passed for place"));
    }

    let data = (fetch_weather_notify(place).await)
        .map_err(|err| err.context("Failed to fetch weather"))?;

    Ok(data)
}

// $ RUST_BACKTRACE=1 mausam
// FIXME: embed default API key for when env is not found.
async fn fetch_weather_notify(query: &str) -> anyhow::Result<OpenWeatherData> {
    let api_var: &str = "WEATHER_API_KEY";
    let weather_api_key: String = {
        let dir: PathBuf = env::current_dir()?;
        let ctx = anyhow!(
            "`{api_var}` environment variable key not found in `{}/.env`",
            dir.to_string_lossy()
        );
        match env::var(api_var).context(ctx) {
            Ok(k) => k,
            Err(err) => {
                log::error!("called `Result::unwrap()` on an `Err` value: {err:#?}");
                eprintln!("{err}: `{}`", err.root_cause());
                std::process::exit(1);
            }
        }
    };
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={query}&appid={weather_api_key}"
    );
    let data: OpenWeatherData = reqwest::get(url)
        .await
        .into_iter()
        .find(|response| is_err_panic(response, query))
        .unwrap()
        .json()
        .await
        .map_err(|e| anyhow!(e).context("Failed to deserialize the response body as JSON."))?;
    {
        let weather = &data
            .weather
            .as_ref()
            .context(anyhow!("Failed to parse weather: {:?}", &data.weather))?
            .first()
            .context("Failed to get first weather vec item")?;

        let weather_description =
            format!("{}{}", &weather.description[..1].to_uppercase(), &weather.description[1..]);
        let main = &data.main;
        let temp = round_f32_dp(temperature::from_k_to_c(main.temp), 2)?;
        let (temp_min, temp_max) = (
            temperature::from_k_to_c(main.temp_min).floor(),
            temperature::from_k_to_c(main.temp_max).ceil(),
        );

        NotifyData::new()
            .with_summary(format!("{query} {temp}°C").as_str())
            .with_body(format!("{weather_description}... {temp_min}°C / {temp_max}°C").as_str())
            .with_icon("weather-few-clouds") // temperature-symbolic. default: alarm
            .show()?;
    }

    Ok(data)
}

fn is_err_panic(response: &Response, query: &str) -> bool {
    if response.status().is_client_error() {
        let err = response.error_for_status_ref().err().unwrap().without_url();
        panic!("{:#?}", anyhow!(err).context(format!("Failed GET request for `{query}`")));
    }
    true
}

#[derive(Debug, Default)]
pub struct NotifyData {
    pub summary: Option<String>,
    pub body: Option<String>,
    pub icon: Option<String>,
    pub appname: String,
    pub timeout: Option<u32>,
    pub hints: Option<Vec<Hint>>,
}

impl NotifyData {
    pub fn new() -> Self {
        Self {
            summary: None,
            body: None,
            icon: None,
            appname: APP_NAME.to_string(),
            timeout: None,
            hints: None,
        }
    }

    /// Sends Notification to D-Bus.
    ///
    /// Returns a handle to a notification
    pub fn show(self) -> anyhow::Result<()> {
        Notification::new()
            .summary(self.summary.unwrap().as_str())
            .body(self.body.unwrap().as_str())
            .icon(self.icon.unwrap().as_str())
            .show()?;
        Ok(())
    }

    /// Set the content of the `body` field.
    ///
    /// Multiline textual content of the notification. Each line should be treated as a paragraph.
    /// Simple html markup should be supported, depending on the server implementation.
    pub fn with_body(mut self, s: &str) -> Self {
        self.body = Some(s.to_string());
        self
    }

    pub fn with_hints(mut self, h: Vec<Hint>) -> Self {
        self.hints = Some(h);
        self
    }

    /// Set the `icon` field.
    ///
    /// You can use common icon names here, usually those in `/usr/share/icons`
    /// can all be used.
    /// You can also use an absolute path to file.
    ///
    /// # Platform support
    ///
    /// macOS does not have support manually setting the icon. However you can pretend to be another app using [`set_application()`](https://docs.rs/notify_rust/4.7.0/notify_rust/notification/fn.set_application.html)
    pub fn with_icon(mut self, s: &str) -> Self {
        self.icon = Some(s.to_string());
        self
    }

    /// Set the `summary`.
    ///
    /// Often acts as title of the notification. For more elaborate content use the `body` field.
    pub fn with_summary(mut self, s: &str) -> Self {
        self.summary = Some(s.to_string());
        self
    }

    pub fn with_timeout(mut self, t: u32) -> Self {
        self.timeout = Some(t);
        self
    }
}

/// Returns a new float with the specified number of decimal points for fractional portion.
/// Rounding currently follows "Bankers Rounding" rules. e.g. 6.5 -> 6, 7.5 -> 8
///
/// # Arguments
/// * `dp`: the number of decimal points to round to.
fn round_f32_dp(num: f32, dp: u32) -> anyhow::Result<f32, ParseFloatError> {
    Decimal::from_f32_retain(num).unwrap().round_dp(dp).to_string().parse::<f32>()
}

mod ipapi {
    use reqwest::{self, Client, Response};
    use serde::{Deserialize, Serialize};

    use super::{CLIENT, IP_API_URL};

    pub(crate) type ResultIpApi = anyhow::Result<IpApiResponse, reqwest::Error>;

    #[derive(Deserialize, Debug, Serialize)]
    pub(crate) struct IpApiResponse {
        pub(crate) status: String,
        pub(crate) country: String,
        #[serde(rename = "countryCode")]
        pub(crate) country_code: String,
        pub(crate) region: String,
        #[serde(rename = "regionName")]
        pub(crate) region_name: String,
        pub(crate) city: String,
        pub(crate) zip: String,
        pub(crate) lat: f64,
        pub(crate) lon: f64,
        pub(crate) timezone: String,
        pub(crate) isp: String, // The ISP name for the location.
        pub(crate) org: String, // The organization name for the location.
        #[serde(rename = "as")]
        pub(crate) as_: String,
    }

    /// `get_location` makes a GET request to the `ip-api` API and retrieves the location
    /// information in JSON format. The JSON response is then deserialized into a struct
    /// `IpApiResponse` using the serde crate.
    pub(crate) async fn get_location(client: &mut Client, url: &str) -> ResultIpApi {
        let response = client.get(url).send().await?;
        let json = response.json::<IpApiResponse>().await?;
        Ok(json)
    }

    /// `get_ip_api_location` fetches the current ip location.
    ///
    /// * Use the `CLIENT` static to make the request to the `IP_API_URL`.
    /// * Extract the JSON from the response and parse it into an `IpApiResponse`.
    /// * Return the parsed JSON as the result of the function.
    pub(crate) async fn get_ip_api_location() -> ResultIpApi {
        let response: Response = (CLIENT).get(*IP_API_URL).send().await?;
        Ok(response.json::<IpApiResponse>().await?)
    }

    /// let location = try_ipapi_location(client, *IP_API_URL).await?;
    /// Note that most geolocation APIs have usage limits,
    /// so be mindful of how often you make requests to the API.
    pub(crate) async fn try_ipapi_location(mut client: Client, url: &str) -> ResultIpApi {
        let location = get_location(&mut client, url).await?;
        println!("Your current location is: {location:?}");
        Ok(location)
    }

    #[test]
    fn should_rename_fields() {
        let api_response = IpApiResponse {
            status: "success".to_string(),
            country: "United States".to_string(),
            country_code: "US".to_string(),
            region: "CA".to_string(),
            region_name: "California".to_string(),
            city: "San Francisco".to_string(),
            zip: "94107".to_string(),
            lat: 37.7749,
            lon: -122.4194,
            timezone: "America/Los_Angeles".to_string(),
            isp: "Google".to_string(),
            org: "Google LLC".to_string(),
            as_: "".to_string(),
        };
        assert_eq!(
            serde_json::to_string(&api_response).unwrap(),
            "{\"status\":\"success\",\"country\":\"United \
             States\",\"countryCode\":\"US\",\"region\":\"CA\",\"regionName\":\"California\",\"\
             city\":\"San \
             Francisco\",\"zip\":\"94107\",\"lat\":37.7749,\"lon\":-122.4194,\"timezone\":\"\
             America/Los_Angeles\",\"isp\":\"Google\",\"org\":\"Google LLC\",\"as\":\"\"}"
        );
    }
}

mod archive {
    use std::net::IpAddr;

    use anyhow::Result;
    use maxminddb::geoip2;

    fn maxminddb_main() -> Result<(), String> {
        let mut args = std::env::args().skip(1);
        let reader = maxminddb::Reader::open_readfile(
            args.next().ok_or("First argument must be the path to the IP database")?,
        )
        .unwrap();
        let ip: IpAddr = args
            .next()
            .ok_or("Second argument must be the IP address, like 128.101.101.101")?
            .parse()
            .unwrap();
        let city: geoip2::City<'_> = reader.lookup(ip).unwrap();
        println!("{city:#?}");
        Ok(())
    }
}
