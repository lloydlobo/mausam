//! # `mausam` - A Rust weather notifier app
//!
//! Mausam is a CLI tool that displays the current weather conditions
//! of your current or any custom location as a desktop notification.
//!
//! ## Features
//!
//! - Cross-platform
//! - Supports arbitrary shell commands.
//! - Constant feedback about the weather updates (`todo` current forecasts).
//!
//! ## Examples
//!
//! By default `$ mausam` will fetch the weather for your current location.
//! Get weather information for Paris, France: `$ mausam paris`
//! Get weather information for London, United Kingdom: `$ mausam london`
//! Get weather information for New York, United States: `$ mausam "new york"`
//!
//! Note: mausam supports various locations from around the world.
//!
//! ## Setup
//!
//! ### API
//!
//! You need to get a API key from [OpenWeatherMap](https://openweathermap.org/api)
//! which is free for a generous request rate puota per day.
//!
//! After subscribing add it to the `.env` file in your root OR home directory
//! as `<your API key>` without `<`/`>`.
//!
//! ```bashls
//! WEATHER_API_KEY=<your API key>
//! ```
//!
//! ## Installation
//!
//! ### Installation Prerequisites
//!
//! Make sure you also have the **development packages of openssl** installed.
//! For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
//!
//! ### From Source
//!
//! ```sh
//! git clone https://github.com/lloydlobo/mausam.git
//! cd mausam
//! cargo build --release
//! cargo install --path .
//! ```
//!
//! ## Usage
//!
//! ### Introduction
//!
//! Mausam is a weather update desktop notifier made with Rust.
//! By default, the temperature unit in the response is in Kelvin, but
//! converted to Celsius before displayed in the notification UI.
//!
//! ### Running Mausam
//!
//! #### Using Your Current Location
//!
//! To display the weather conditions of your current location,
//! run the following command in your terminal:
//!
//! ```sh
//! mausam
//! ```
//!
//! #### Running Mausam Periodically with Cron
//!
//! Schedule cron jobs to run on a time interval for the current user.
//! More information: [https://crontab.guru/](https://crontab.guru/).
//!
//! ##### Cron Prerequisites
//!
//! Before you can run Mausam periodically with cron, you must meet the following requirements:
//!
//! - Have `crontab` installed on your system
//! - Have Mausam installed on your system, with `Cargo`, the Rust's toolchain.
//!   - After installation, /home/<YOUR_USER_NAME>/.cargo/bin/` contains the binary
//!   by default.
//! - Place the `.env` file in the `path/to/mausam` directory, as this file holds the secret
//!   `WEATHER_API_KEY`.
//!
//! ##### Usage with `crontab`
//!
//! To run `mausam` every 60 minutes using `crontab`, follow these steps:
//!
//! - Edit the crontab file for the current user:
//!
//!   ```sh
//!   crontab -e
//!   ```
//!
//! - Add the following line to the file:
//!
//!   ```crontab
//!   # Run mausam (weather notification cli) every 60 minutes
//!   */60 * * * * cd ~/path/to/mausam/ && ~/.cargo/bin/mausam
//!   ```
//!
//! - Save the file and exit your editor.
//!
//! - Check the status of `crontab` with `systemctl`:
//!
//!   ```sh
//!   systemctl status crond.service
//!   ```
//!
//! By following these steps, you can have the current weather conditions of your
//! location displayed as a desktop notification every hour.
//!
//! ### Terminal output with API response
//!
//! ```sh
//! $ mausam paris
//! {
//!   "coord": {
//!     "lon": 2.3488,
//!     "lat": 48.8534
//!   },
//!   "weather": [
//!     {
//!       "id": 804,
//!       "main": "Clouds",
//!       "description": "overcast clouds",
//!       "icon": "04d"
//!     }
//!   ],
//!   "base": "stations",
//!   "main": {
//!     "temp": 283.18,
//!     "feels_like": 282.12,
//!     "temp_min": 282.58,
//!     "temp_max": 283.92,
//!     "pressure": 1031,
//!     "humidity": 72
//!   },
//!   "visibility": 10000,
//!   "wind": {
//!     "speed": 6.17,
//!     "deg": 260
//!   },
//!   "clouds": {
//!     "all": 100
//!   },
//!   "dt": 1675343032,
//!   "sys": {
//!     "type": 2,
//!     "id": 2041230,
//!     "country": "FR",
//!     "sunrise": 1675322401,
//!     "sunset": 1675356505
//!   },
//!   "timezone": 3600,
//!   "id": 2988507,
//!   "name": "Paris",
//!   "cod": 200
//! }
//! ```
//!
//! ## Dev
//!
//! ### Test
//!
//! ```sh
//! CARGO_LOG=error cargo test
//! ```
//!
//! ### PERF
//!
//! ```sh
//! RUST_BACKTRACE=1 mausam
//! ```
//!
//! ## Troubleshooting
//!
//! ### API key doesn't work right after subscribing
//!
//! > Your API key is not activated yet. Within the next couple of hours,
//! > it will be activated and ready to use.
//! > `https://api.openweathermap.org/data/2.5/weather?lat=44.34&lon=10.99&appid={API key}`
//!
//! If you still face issues or have questions, open an issue on the GitHub repository.
//! The maintainers will be happy to help.
//!
//! ## Origin of the name
//!
//! The name `mausam` is in reference to the borrowed name from
//! Persian `موسم` (mousem), and from Arabic `مَوْسِم`(mawsim). which means weather.
//!
//! ## License
//!
//! `mausam` is dual-licensed under the terms of the MIT License
//! and the Apache License 2.0.
//!
//! See the [LICENSE-APACHE](LICENSE-APACHE)
//! and [LICENSE-MIT](LICENSE-MIT) files for details.

mod app;
mod cli;
mod models;

use anyhow::{anyhow, Context};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    match app::run().await {
        Ok(v) => Ok(println!("{}", serde_json::to_string_pretty(&v)?)),
        Err(e) => {
            let context = anyhow::format_err!(
                "Failed to run at `{}`: `{:#?}`",
                std::env::current_dir()?
                    .to_str()
                    .context(anyhow!("Failed to find current_dir.\n>> Trace: {:#?}", e))
                    .unwrap_err(),
                e,
            );
            eprintln!("{:#?}", e.context(context));
            std::process::exit(1)
        }
    }
}

// $ RUST_BACKTRACE=1 mausam
// $ CARGO_LOG=trace cargo run

// NOTE: Docker container has .env file. Remove it before pushing ot docker hub

// TODO:
// ./mausam: error while loading shared libraries: libssl.so.1
// .1: cannot open shared object file: No such file or directory
