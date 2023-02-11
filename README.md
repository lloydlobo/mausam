# mausam

[![CICD](https://github.com/lloydlobo/mausam/actions/workflows/CICD.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/CICD.yml)
[![Deploy Pages](https://github.com/lloydlobo/mausam/actions/workflows/docs.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/docs.yml)
[![rust-clippy analyze](https://github.com/lloydlobo/mausam/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/rust-clippy.yml)

Mausam is a CLI tool that displays the current weather conditions of your location as a desktop notification.

![mausam](https://github.com/lloydlobo/mausam/blob/master/assets/demo.gif)

## Features

- Cross-platform
- Arbitrary shell commands are supported.
- Constant feedback about the weather updates (..and TODO: current forecasts).
<!-- TODO: * Export results to various formats: CSV, JSON, Markdown, AsciiDoc. -->

## Usage

### Introduction

Mausam is a a weather update desktop notifier made with Rust.
By default, the temperature unit in the response is in Kelvin, but it is converted to Celsius before being displayed in the notification.

### Running Mausam

#### Using Your Current Location

To display the weather conditions of your current location, simply run the following command in your terminal:

```sh
$ mausam
```

#### Running Mausam Periodically with Cron

To run Mausam periodically, you can use `cron` via `crontab`.
You will need to have Mausam installed on your system, which can be done using `cargo`, the Rust's toolchain.
After installation, the binary will be stored in `/home/<YOUR_USER_NAME>/.cargo/bin/` by default.

Make sure to place the `.env` file in the `path/to/mausam` directory, as this file holds the secret `WEATHER_API_KEY`.

Here's an example of how to run `mausam` every 60 minutes using `crontab`:

```crontab
# Run mausam (weather notification cli) every 60 minutes
*/60 * * * * cd ~/path/to/mausam/ && ~/.cargo/bin/mausam
```

By following these steps, you can have the current weather conditions of your location, displayed as a desktop notification every hour.

### Terminal output with API response

```sh
$ mausam paris
{
  "coord": {
    "lon": 2.3488,
    "lat": 48.8534
  },
  "weather": [
    {
      "id": 804,
      "main": "Clouds",
      "description": "overcast clouds",
      "icon": "04d"
    }
  ],
  "base": "stations",
  "main": {
    "temp": 283.18,
    "feels_like": 282.12,
    "temp_min": 282.58,
    "temp_max": 283.92,
    "pressure": 1031,
    "humidity": 72
  },
  "visibility": 10000,
  "wind": {
    "speed": 6.17,
    "deg": 260
  },
  "clouds": {
    "all": 100
  },
  "dt": 1675343032,
  "sys": {
    "type": 2,
    "id": 2041230,
    "country": "FR",
    "sunrise": 1675322401,
    "sunset": 1675356505
  },
  "timezone": 3600,
  "id": 2988507,
  "name": "Paris",
  "cod": 200
}
```

## Setup

### API

You may get a API key from [OpenWeatherMap](https://openweathermap.org/api) for
free for a generous request rate a day.

After subscribing add it to the `.env` file as `<your API key>` without `<`/`>`.

```bashls
WEATHER_API_KEY=<your API key>
```

## Installation

### From Source

```sh
git clone https://github.com/lloydlobo/mausam.git
cd mausam
cargo build --release
cargo install --path .
```

## Dev

### Test

```sh
CARGO_LOG=error cargo test
```

### PERF

```sh
$ RUST_BACKTRACE=1 mausam
```

## Troubleshooting

### API key doesn't work immediately after subscribing

> Your API key is not activated yet. Within the next couple of hours, it will be activated and ready to use. https://api.openweathermap.org/data/2.5/weather?lat=44.34&lon=10.99&appid={API key}

## Origin of the name

The name `mausam` was chosen in reference to the borrowed name
from Persian `موسم` (mousem), and from Arabic `مَوْسِم`(mawsim). which means weather.

## License

`mausam` is dual-licensed under the terms of the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for details.
