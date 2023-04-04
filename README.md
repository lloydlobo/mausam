# mausam

[![CICD](https://github.com/lloydlobo/mausam/actions/workflows/CICD.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/CICD.yml)
[![Deploy Pages](https://github.com/lloydlobo/mausam/actions/workflows/docs.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/docs.yml)
[![rust-clippy analyze](https://github.com/lloydlobo/mausam/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/lloydlobo/mausam/actions/workflows/rust-clippy.yml)

Mausam is a CLI tool that displays the current weather conditions
of your current or any custom location as a desktop notification.

![mausam](https://github.com/lloydlobo/mausam/blob/master/assets/demo.gif)

<!--toc:start-->
- [mausam](#mausam)
  - [Features](#features)
  - [Setup](#setup)
    - [API](#api)
  - [Installation](#installation)
    - [Installation Prerequisites](#installation-prerequisites)
    - [From Source](#from-source)
  - [Usage](#usage)
    - [Introduction](#introduction)
    - [Running Mausam](#running-mausam)
      - [Using Your Current Location](#using-your-current-location)
      - [Running Mausam Periodically with Cron](#running-mausam-periodically-with-cron)
        - [Cron Prerequisites](#cron-prerequisites)
        - [Usage with `crontab`](#usage-with-crontab)
    - [Terminal output with API response](#terminal-output-with-api-response)
  - [Dev](#dev)
    - [Test](#test)
    - [PERF](#perf)
  - [Troubleshooting](#troubleshooting)
    - [API key doesn't work right after subscribing](#api-key-doesnt-work-right-after-subscribing)
  - [Origin of the name](#origin-of-the-name)
  - [License](#license)
<!--toc:end-->

## Features

- Cross-platform
- Supports arbitrary shell commands.
- Constant feedback about the weather updates (`todo` current forecasts).

## Setup

### API

You need to get a API key from [OpenWeatherMap](https://openweathermap.org/api)
which is free for a generous request rate puota per day.

After subscribing add it to the `.env` file in your root OR home directory
as `<your API key>` without `<`/`>`.

```bashls
WEATHER_API_KEY=<your API key>
```

## Installation

### Installation Prerequisites

Make sure you also have the **development packages of openssl** installed.
For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

### From Source

```sh
git clone https://github.com/lloydlobo/mausam.git
cd mausam
cargo build --release
cargo install --path .
```

## Usage

### Introduction

Mausam is a weather update desktop notifier made with Rust.
By default, the temperature unit in the response is in Kelvin, but
converted to Celsius before displayed in the notification UI.

### Running Mausam

#### Using Your Current Location

To display the weather conditions of your current location,
run the following command in your terminal:

```sh
mausam
```

#### Running Mausam Periodically with Cron

Schedule cron jobs to run on a time interval for the current user.
More information: [https://crontab.guru/](https://crontab.guru/).

##### Cron Prerequisites

Before you can run Mausam periodically with cron, you must meet the following requirements:

- Have `crontab` installed on your system
- Have Mausam installed on your system, with `Cargo`, the Rust's toolchain.
  - After installation, /home/<YOUR_USER_NAME>/.cargo/bin/` contains the binary
  by default.
- Place the `.env` file in the `path/to/mausam` directory, as this file holds
  the secret `WEATHER_API_KEY`.

##### Usage with `crontab`

To run `mausam` every 60 minutes using `crontab`, follow these steps:

- Edit the crontab file for the current user:

  ```sh
  crontab -e
  ```

- Add the following line to the file:

  ```crontab
  # Run mausam (weather notification cli) every 60 minutes
  */60 * * * * cd ~/path/to/mausam/ && ~/.cargo/bin/mausam
  ```

- Save the file and exit your editor.

- Check the status of `crontab` with `systemctl`:

  ```sh
  systemctl status crond.service
  ```

By following these steps, you can have the current weather conditions of your
location displayed as a desktop notification every hour.

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

## Dev

### Test

```sh
CARGO_LOG=error cargo test
```

### PERF

```sh
RUST_BACKTRACE=1 mausam
```

## Troubleshooting

### API key doesn't work right after subscribing

> Your API key is not activated yet. Within the next couple of hours,
> it will be activated and ready to use.
> `https://api.openweathermap.org/data/2.5/weather?lat=44.34&lon=10.99&appid={API key}`

## Origin of the name

The name `mausam` is in reference to the borrowed name from
Persian `موسم` (mousem), and from Arabic `مَوْسِم`(mawsim). which means weather.

## License

`mausam` is dual-licensed under the terms of the MIT License
and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE)
and [LICENSE-MIT](LICENSE-MIT) files for details.
