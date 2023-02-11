//! mausam - A Rust weather notifier app
//!
//! mausam provides current weather updates for a given location.
//!
//! ## Features
//!
//! - Cross-platform support.
//! - Runs shell commands.
//! - Constantly updated weather info, including current forecast.
//!
//! ## Usage
//!
//! Terminal outputs API response in Kelvin by default, but UI notifications are in Celsius.
//! To get weather info for a location, run:
//!
//! //! $ mausam <location> //!
//!
//! Output Example:
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
//! ## Setup
//!
//! To use mausam, you need to obtain an API key from `OpenWeatherMap`. A free account provides a
//! generous request rate per day. Once subscribed, add the API key to your .env file:
//!
//! //! `WEATHER_API_KEY`=<API key> //!
//!
//! ## Installation
//!
//! To install mausam from source, follow these steps:
//!
//! //! git clone https://github.com/lloydlobo/mausam.git //! cd mausam //! cargo build --release //! cargo install --path . //!
//!
//! ## Development
//!
//! To test the code, run:
//!
//! //! `CARGO_LOG=error` cargo test //!
//!
//! ## Troubleshooting
//!
//! If your API key doesn't work immediately after subscribing, wait a couple of hours for
//! activation. You can verify activation status by accessing the API at:
//!
//! //! https://api.openweathermap.org/data/2.5/weather?lat=44.34&lon=10.99&appid={API key} //!
//!
//! If you still face issues or have questions, open an issue on the GitHub repository.
//! The maintainers will be happy to help.
//!
//! ## Examples
//!
//! Get weather information for Paris, France:
//!
//! //! $ mausam paris //!
//!
//! Get weather information for London, United Kingdom:
//!
//! //! $ mausam london //!
//!
//! Get weather information for New York, United States:
//!
//! //! $ mausam new york //!
//!
//! Note: mausam supports various locations from around the world.
//!
//! ## Origin of the name `mausam`
//!
//! The name mausam is borrowed from Persian "موسم" (mousem) and Arabic "مَوْسِم" (mawsim), meaning
//! "weather".

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
//
