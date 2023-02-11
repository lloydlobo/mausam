use std::time::{Duration, SystemTime};

use toml;

lazy_static! {
    static ref LAST_UPDATED: std::sync::Mutex<SystemTime> =
        std::sync::Mutex::new(SystemTime::now());
    static ref LOCATION: std::sync::Mutex<Option<Location>> = std::sync::Mutex::new(None);
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
    accuracy: f64,
}

impl Location {
    fn from_ipapi() -> anyhow::Result<Location, reqwest::Error> {
        let response = Client::new().get("http://ip-api.com/json").send()?;
        let json: IpApiResponse = response.json()?;
        Ok(Location { latitude: json.lat, longitude: json.lon, accuracy: json.accuracy })
    }

    fn from_config() -> anyhow::Result<Option<Location>, toml::de::Error> {
        let config = std::fs::read_to_string("config.toml")?;
        let config: toml::Value = toml::from_str(&config)?;
        let location =
            config.get("location").and_then(toml::Value::as_table).map(|location| Location {
                latitude: location.get("latitude").and_then(toml::Value::as_float).unwrap_or(0.0),
                longitude: location.get("longitude").and_then(toml::Value::as_float).unwrap_or(0.0),
                accuracy: location.get("accuracy").and_then(toml::Value::as_float).unwrap_or(0.0),
            });
        Ok(location)
    }

    fn save_to_config(&self) -> anyhow::Result<(), toml::ser::Error> {
        let config = toml::to_string(self)?;
        std::fs::write("config.toml", config)?;
        Ok(())
    }
}

pub fn get_location() -> anyhow::Result<Location, Box<dyn std::error::Error>> {
    let last_updated = LAST_UPDATED.lock().unwrap();
    let location = LOCATION.lock().unwrap();
    if let Some(location) = &*location {
        if last_updated.elapsed()? < Duration::from_secs(60 * 60 * 24) {
            return Ok(*location);
        }
    }
    let location = match Location::from_config() {
        Ok(Some(location)) => location,
        _ => Location::from_ipapi()?,
    };
    *last_updated = SystemTime::now();
    *location = Some(location);
    location.save_to_config()?;
    Ok(location)
}
//To check the accuracy of the location stored in the config file, it is recommended to verify it
// against the location data obtained from a trusted source, such as the ip-api.com API.
//
//One approach to verifying the location stored in the config file would be to compare it against
// the location data obtained from the API, and if there is a significant discrepancy, updating the
// config file with the newly obtained location data. This verification process can be done
// periodically, for example, every time the application is launched, or at a specific interval,
// like once a week.
//
//Alternatively, if real-time location accuracy is critical for the application, it can be verified
// with the API each time it is required to use the location data. This approach would increase the
// number of API requests and might result in higher latency, but would ensure that the location
// data is always up-to-date and accurate. Here's a simple program that demonstrates how to fetch
// location information using the ipapi API, save it to a configuration file using the toml crate,
// and then retrieve and check the location information from the config file:
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
    city: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    location: Location,
}

const IP_API_URL: &str = "http://ip-api.com/json";
const CONFIG_FILE: &str = "config.toml";

// Fetch location information from the ip-api.com API
async fn fetch_location() -> Result<Location, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(IP_API_URL).send().await?;
    let location: Location = response.json().await?;
    Ok(location)
}

// Save location information to the config file
fn save_config(location: &Location) -> Result<(), std::io::Error> {
    let config = Config { location: location.clone() };
    let config_string = toml::to_string(&config)?;
    std::fs::write(CONFIG_FILE, config_string)?;
    Ok(())
}

// Load location information from the config file
fn load_config() -> Result<Location, toml::de::Error> {
    let config_string = std::fs::read_to_string(CONFIG_FILE)?;
    let config: Config = toml::from_str(&config_string)?;
    Ok(config.location)
}
/// In this program, the location information is fetched from the ipapi API and saved to a
/// configuration file using the toml crate. The location information is stored in a struct named
/// Location. The program first tries to load the location information from the config file. If the
/// config file doesn't exist or can't be parsed, the program fetches the location information again
/// from the API and saves it to
#[tokio::main]
async fn try_fetcher_main() {
    // First, try to load location information from the config file
    let location = match load_config() {
        Ok(location) => location,
        Err(_) => {
            // If the config file doesn't exist or can't be parsed, fetch the location information
            let location = fetch_location().await.unwrap();
            // And then save it to the config file
            save_config(&location).unwrap();
            location
        }
    };

    println!("Location information: {:?}", location);

    // Check if the location information is correct by fetching it again from the API
    let current_location = fetch_location().await.unwrap();
    if location == current_location {
        println!("Location information is up-to-date and correct");
    } else {
        println!("Location information is outdated, updating...");
        save_config(&current_location).unwrap();
    }
}

mod verify {
    //! This program will check the location of the user every hour and update it if necessary. The
    //! location is stored in a Config struct, which is read from a configuration file. If the
    //! location retrieved from ip-api.com is different from the stored location, the stored
    //! location is updated with the new location.

    use anyhow::Result;
    use maxminddb::{geoip2::Country, Reader};
    use reqwest::Client;
    use serde::Deserialize;
    use tokio::time::interval;

    #[derive(Debug, Deserialize)]
    struct Config {
        location: String,
    }

    #[derive(Debug, Deserialize)]
    struct IpApiResponse {
        country: String,
    }

    async fn verify_location(config: &Config) -> Result<String> {
        let client = Client::new();
        let response = client.get("http://ip-api.com/json").send().await?;
        let json: IpApiResponse = response.json().await?;
        if config.location != json.country {
            Ok(json.country)
        } else {
            Ok(config.location)
        }
    }

    #[tokio::main]
    async fn main() -> Result<()> {
        let config = Config { location: "US".to_owned() };

        let interval = interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;

            let new_location = verify_location(&config).await?;
            if new_location != config.location {
                println!("Updating location: {} -> {}", config.location, new_location);
                config.location = new_location;
            }
        }
    }
}
