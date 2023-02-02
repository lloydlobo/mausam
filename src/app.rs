//! Library for `mausam`
// $ RUST_BACKTRACE=1 mausam

mod temperature;

use std::{env, num::ParseFloatError, path::PathBuf};

use anyhow::{anyhow, Context};
use clap::Parser;
use dotenv::dotenv;
use notify_rust::{Hint, Notification};
use reqwest::Response;
use rust_decimal::Decimal;

use crate::{cli::Cli, models::OpenWeatherData};

// HACK: Can use RUST_PACKAGE name env?
pub const APP_NAME: &str = "mausam";

pub async fn run() -> anyhow::Result<OpenWeatherData> {
    dotenv().ok();
    let mut args = Cli::parse();
    let place = args.place.get_or_insert("London".to_string());
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
