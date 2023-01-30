//! Library for `mausam`

mod models;

use std::{env, num::ParseFloatError};

use dotenv::dotenv;
use notify_rust::{Hint, Notification};
use reqwest::Response;
use rust_decimal::Decimal; // use rust_decimal_macros::*;

use crate::models::OpenWeatherData;

// HACK: Can use RUST_PACKAGE name env?
pub const APP_NAME: &str = "mausam";

pub async fn run() -> anyhow::Result<()> {
    dotenv().ok();
    handle_get_notify_weather().await?;
    Ok(())
}

// Your API key is not activated yet. Within the next couple of hours, it will be activated and
// ready to use. https://api.openweathermap.org/data/2.5/weather?lat=44.34&lon=10.99&appid={API key}
async fn handle_get_notify_weather() -> anyhow::Result<()> {
    let weather_api_key: String =
        env::vars().find(|(key, _)| key == "WEATHER_API_KEY").map(|(_, v)| v).unwrap();
    let city: &str = "London";
    let api_city =
        format!("https://api.openweathermap.org/data/2.5/weather?q={city}&appid={weather_api_key}");
    let req: Response = reqwest::get(api_city).await?;
    let resp: OpenWeatherData = req.json().await?;

    let weather = &resp.weather.unwrap();
    let weather = &weather.first().unwrap();
    let weather_description =
        format!("{}{}", &weather.description[..1].to_uppercase(), &weather.description[1..]);
    let main = &resp.main;
    let temp = round_f32_dp(from_f_to_cel(main.temp), 2)?;
    let (temp_min, temp_max) =
        (from_f_to_cel(main.temp_min).floor(), from_f_to_cel(main.temp_max).ceil());

    NotifyData::new()
        .with_summary(format!("{city} {temp}°C").as_str())
        .with_body(format!("{weather_description}... {temp_min}°C / {temp_max}°C").as_str())
        .with_icon("weather-few-clouds") // temperature-symbolic. default: alarm
        .show()?;

    Ok(())
}

/// Convert degrees Fahrenheit to degrees Celsius.
///
/// Formula - `(33.8°F − 32) × 5/9 = 1°C`
fn from_f_to_cel(f: f32) -> f32 {
    const CONSTANT_F_TO_C: f32 = 33.8;

    f / CONSTANT_F_TO_C
}

/// Returns a new float with the specified number of decimal points for fractional portion.
/// Rounding currently follows "Bankers Rounding" rules. e.g. 6.5 -> 6, 7.5 -> 8
///
/// # Arguments
/// * `dp`: the number of decimal points to round to.
fn round_f32_dp(num: f32, dp: u32) -> anyhow::Result<f32, ParseFloatError> {
    Decimal::from_f32_retain(num).unwrap().round_dp(dp).to_string().parse::<f32>()
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
            .summary(&self.summary.unwrap().as_str())
            .body(&self.body.unwrap().as_str())
            .icon(&self.icon.unwrap().as_str())
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

// pub enum NotifyType {
//     Simple,
//     Persistent,
// }
//
// impl Default for NotifyType {
//     fn default() -> Self {
//         Self::Simple
//     }
// }
//
// // fn notify(n: NotifyType) -> anyhow::Result<()> {
//     match n {
//         NotifyType::Simple => {
//             Notification::new()
//                 .summary("Firefox News")
//                 .body("This will almost look like a real firefox notification.")
//                 .icon("firefox")
//                 .show()?;
//         }
//         NotifyType::Persistent => {
//             Notification::new()
//                 .summary("Category:email")
//                 .body(
//                     "This has nothing to do with emails.\nIt should not go away until you \
//                      acknowledge it.",
//                 )
//                 .icon("thunderbird")
//                 .appname("thunderbird")
//                 .hint(Hint::Category("email".to_owned()))
//                 .hint(Hint::Resident(true)) // this is not supported by all implementations
//                 .timeout(0) // this however is
//                 .show()?;
//         }
//     }
//     Ok(())
// }
