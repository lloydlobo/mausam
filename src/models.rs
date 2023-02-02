use serde::{Deserialize, Serialize};

/// JSON API response structure to expect from Open Weather API.
/// [Reference](https://openweathermap.org/current)
// Get types from... https://jvilk.com/MakeTypes/
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OpenWeatherData {
    pub coord: Coord,
    pub weather: Option<Vec<WeatherEntity>>,
    pub base: String,
    pub main: Main,
    pub visibility: i32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i32,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

impl Default for OpenWeatherData {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenWeatherData {
    pub fn new() -> Self {
        Self {
            coord: Coord { lon: -0.1257_f32, lat: 51.5085_f32 },
            weather: Some(vec![WeatherEntity {
                id: 803,
                main: "Clouds".to_string(),
                description: "broken clouds".to_string(),
                icon: " 04n".to_string(),
            }]),
            base: "stations".to_string(),
            main: Main {
                temp: 280.34_f32,
                feels_like: 276.76_f32,
                temp_min: 278.64_f32,
                temp_max: 281.62_f32,
                pressure: 1021,
                humidity: 86,
            },
            visibility: 10_000,
            wind: Wind { speed: 6.17_f32, deg: 300 },
            clouds: Clouds { all: 75 },
            dt: 1675061138,
            sys: Sys {
                type_sys: 2,
                id: 2075535,
                country: "GB".to_string(),
                sunrise: 1675064547,
                sunset: 1675097090,
            },
            timezone: 0,
            id: 2643743,
            name: "London".to_string(),
            cod: 200,
        }
    }

    pub fn with_coord(mut self, coord: Coord) -> Self {
        self.coord = coord;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WeatherEntity {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Main {
    /// `main.temp` Temperature.
    /// Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Wind {
    pub speed: f32,
    pub deg: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_sys: i32,
    pub id: i32,
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32,
}

// ❯ CARGO_LOG=error cargo test
#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use pretty_assertions::assert_eq;

    use super::*;

    const EXPECT_RESPONSE: &str = r#"{"coord":{"lon":-0.1257,"lat":51.5085},"weather":[{"id":803,"main":"Clouds","description":"broken clouds","icon":" 04n"}],"base":"stations","main":{"temp":280.34,"feels_like":276.76,"temp_min":278.64,"temp_max":281.62,"pressure":1021,"humidity":86},"visibility":10000,"wind":{"speed":6.17,"deg":300},"clouds":{"all":75},"dt":1675061138,"sys":{"type":2,"id":2075535,"country":"GB","sunrise":1675064547,"sunset":1675097090},"timezone":0,"id":2643743,"name":"London","cod":200}"#;

    #[test]
    fn should_json_to_string() {
        let got: String = serde_json::to_string(&OpenWeatherData::default())
            .map_err(|e| anyhow!("Failed to convert from json to string: {e}"))
            .unwrap();
        assert_eq!(got, EXPECT_RESPONSE);
    }

    #[test]
    fn should_match_response_struct() {
        let got = OpenWeatherData::default();
        let expect: OpenWeatherData = serde_json::from_str(EXPECT_RESPONSE)
            .map_err(|e| anyhow!("Failed to convert to json from string: {e}"))
            .unwrap();
        assert_eq!(got, expect);
    }
}
