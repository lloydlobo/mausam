# mausam

[![CICD](https://github.com/lloydlobo/mausam/actions/workflows/CICD.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/CICD.yml)

A weather update desktop notifier made with Rust.

![mausam](https://github.com/lloydlobo/mausam/blob/master/assets/demo.gif)

## Features

- Cross-platform
- Arbitrary shell commands are supported.
- Constant feedback about the weather updates (..and TODO: current forecasts).
<!-- TODO: * Export results to various formats: CSV, JSON, Markdown, AsciiDoc. -->

## Usage

```sh
$ mausam
London 8.3°C
Broken clouds... 8°C/9°C
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

## Troubleshooting

### API key doesn't work immediately after subscribing

> Your API key is not activated yet. Within the next couple of hours, it will be activated and ready to use. https://api.openweathermap.org/data/2.5/weather?lat=44.34&lon=10.99&appid={API key}

## Origin of the name

The name `mausam` was chosen in reference to the borrowed name
from Persian `موسم` (mousem), and from Arabic `مَوْسِم`(mawsim). which means weather.

## License

`mausam` is dual-licensed under the terms of the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for details.
