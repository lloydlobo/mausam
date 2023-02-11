# mausam

[![CICD](https://github.com/lloydlobo/mausam/actions/workflows/CICD.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/CICD.yml)
[![Deploy Pages](https://github.com/lloydlobo/mausam/actions/workflows/docs.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/docs.yml)
[![rust-clippy analyze](https://github.com/lloydlobo/mausam/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/rust-clippy.yml)

A weather update desktop notifier made with Rust.

![mausam](https://github.com/lloydlobo/mausam/blob/master/assets/demo.gif)

## Features

- Cross-platform
- Arbitrary shell commands are supported.
- Constant feedback about the weather updates (..and TODO: current forecasts).
<!-- TODO: * Export results to various formats: CSV, JSON, Markdown, AsciiDoc. -->

## Usage

### Terminal output with API response

NOTE: Default temperature is of the unit `Kelvin`.
The response for the desktop ui notification is converted to relative `Celsius` temperature units.

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
