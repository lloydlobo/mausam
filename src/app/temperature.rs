use std::{convert::From, fmt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[macro_export]
macro_rules! display_tempunit {
    ($unit:expr) => {
        match $unit {
            TempUnit::Fahrenheit => "°F",
            TempUnit::Celsius => "°C",
            TempUnit::Kelvin => "K",
        }
    };
}

impl fmt::Display for TempUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fahrenheit => write!(f, "°F"),
            Self::Celsius => write!(f, "°C"),
            Self::Kelvin => write!(f, "K"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    pub value: f32,
    pub unit: TempUnit,
}

impl From<Temperature> for f32 {
    fn from(temp: Temperature) -> f32 {
        temp.value
    }
}

impl From<f32> for Temperature {
    fn from(val: f32) -> Self {
        Temperature { value: val, unit: TempUnit::Celsius }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

impl Temperature {
    pub fn new(value: f32, unit: TempUnit) -> Self {
        Self { value, unit }
    }

    pub fn to_fahrenheit(&self) -> Temperature {
        match self.unit {
            TempUnit::Fahrenheit => Temperature { value: self.value, unit: TempUnit::Fahrenheit },
            // Convert degrees Celsius to degrees Fahrenheit. Formula - `(33.8°F − 32) × 5/9 = 1°C`
            TempUnit::Celsius => Temperature {
                value: (self.value * (9f32 / 5f32)) + 32f32,
                unit: TempUnit::Fahrenheit,
            },
            // Convert Kelvin to degrees Fahrenheit.
            TempUnit::Kelvin => Temperature {
                value: (self.value - 273.15_f32) * (9f32 / 5f32) + 32f32,
                unit: TempUnit::Fahrenheit,
            },
        }
    }

    pub fn to_celsius(&self) -> Temperature {
        match self.unit {
            // Convert degrees Fahrenheit to degrees Celsius. Formula - `(33.8°F − 32) × 5/9 = 1°C`
            TempUnit::Fahrenheit => {
                Temperature { value: (self.value - 32f32) * (5f32 / 9f32), unit: TempUnit::Celsius }
            }
            TempUnit::Celsius => Temperature { value: self.value, unit: TempUnit::Celsius },
            // Convert Kelvin to degrees Celsius. Formula - `0K − 273.15 = -273.1°C`
            TempUnit::Kelvin => {
                Temperature { value: self.value - 273.15_f32, unit: TempUnit::Celsius }
            }
        }
    }

    pub fn to_kelvin(&self) -> Temperature {
        match self.unit {
            // Convert degrees Fahrenheit to Kelvin.
            TempUnit::Fahrenheit => Temperature {
                value: (self.value - 32f32) * (5f32 / 9f32) + 273.15_f32,
                unit: TempUnit::Kelvin,
            },
            // Convert degrees Celsius to Kelvin.
            TempUnit::Celsius => {
                Temperature { value: self.value + 273.15_f32, unit: TempUnit::Kelvin }
            }
            TempUnit::Kelvin => Temperature { value: self.value, unit: TempUnit::Kelvin },
        }
    }
}

#[cfg(test)]
pub mod tests {
    // Formula - `0K − 273.15 = -273.1°C`
    // Formula - `(33.8°F − 32) × 5/9 = 1°C`
    // Formula - `0K − 273.15 = -273.1°C`
    // Formula - `0K − 273.15 = -273.1°C`
    // Formula - `(33.8°F − 32) × 5/9 = 1°C`
    use pretty_assertions::assert_eq;
    use quickcheck::{quickcheck, Arbitrary, Gen};

    use super::*;

    impl Arbitrary for Temperature {
        fn arbitrary(g: &mut Gen) -> Self {
            // An array of possible temperatures to choose from.
            let temp = &[-273.15, -100.0f32, -50.0, 0.0, 20.0, 50.0, 100.0, 273.15];
            // Choose a temperature value from the array randomly.
            let value = g.choose(temp).unwrap().to_owned();
            // An array of possible temperature units to choose from.
            let units = &[TempUnit::Fahrenheit, TempUnit::Celsius, TempUnit::Kelvin];
            // Choose a temperature unit from the array randomly.
            let unit = *g.choose(units).unwrap();
            // Return a new instance of `Temperature` with the randomly chosen value and unit.
            Temperature { value, unit }
        }
    }

    #[test]
    fn test_to_fahrenheit() {
        let celsius = Temperature::new(0.0, TempUnit::Celsius);
        let kelvin = Temperature::new(273.15, TempUnit::Kelvin);
        let fahrenheit = Temperature::new(32.0, TempUnit::Fahrenheit);

        let celsius_to_fahrenheit = celsius.to_fahrenheit();
        let kelvin_to_fahrenheit = kelvin.to_fahrenheit();
        let fahrenheit_to_fahrenheit = fahrenheit.to_fahrenheit();

        assert_eq!(celsius_to_fahrenheit.value, 32.0);
        assert_eq!(celsius_to_fahrenheit.unit, TempUnit::Fahrenheit);
        assert_eq!(kelvin_to_fahrenheit.value, 32.0);
        assert_eq!(kelvin_to_fahrenheit.unit, TempUnit::Fahrenheit);
        assert_eq!(fahrenheit_to_fahrenheit.value, 32.0);
        assert_eq!(fahrenheit_to_fahrenheit.unit, TempUnit::Fahrenheit);
    }

    #[test]
    fn test_to_celsius() {
        let fahrenheit = Temperature::new(32.0, TempUnit::Fahrenheit);
        let kelvin = Temperature::new(273.15, TempUnit::Kelvin);
        let celsius = Temperature::new(0.0, TempUnit::Celsius);

        let fahrenheit_to_celsius = fahrenheit.to_celsius();
        let kelvin_to_celsius = kelvin.to_celsius();
        let celsius_to_celsius = celsius.to_celsius();

        assert_eq!(fahrenheit_to_celsius.value, 0.0);
        assert_eq!(fahrenheit_to_celsius.unit, TempUnit::Celsius);
        assert_eq!(kelvin_to_celsius.value, 0.0);
        assert_eq!(kelvin_to_celsius.unit, TempUnit::Celsius);
        assert_eq!(celsius_to_celsius.value, 0.0);
        assert_eq!(celsius_to_celsius.unit, TempUnit::Celsius);
    }

    #[test]
    fn test_to_kelvin() {
        let celsius = Temperature::new(0.0, TempUnit::Celsius);
        let kelvin = celsius.to_kelvin();
        assert_eq!(kelvin.value, 273.15);
        assert_eq!(kelvin.unit, TempUnit::Kelvin);

        let fahrenheit = Temperature::new(32.0, TempUnit::Fahrenheit);
        let kelvin = fahrenheit.to_kelvin();
        assert_eq!(kelvin.value, 273.15 + (32.0 - 32.0) * (5.0 / 9.0));
        assert_eq!(kelvin.unit, TempUnit::Kelvin);

        let kelvin = Temperature::new(0.0, TempUnit::Kelvin);
        let kelvin_ = kelvin.to_kelvin();
        assert_eq!(kelvin_.value, 0.0);
        assert_eq!(kelvin_.unit, TempUnit::Kelvin);
    }

    // In this example, the quickcheck function is used to generate random Temperature values and
    // pass them to the property functions, which test the correctness of the to_kelvin, to_celsius,
    // and to_fahrenheit functions. The property functions return a boolean indicating whether the
    // test passed or failed.
    #[test]
    fn test_to_celsius_conversion() {
        fn prop(temp: Temperature) -> bool {
            let expected = match temp.unit {
                TempUnit::Fahrenheit => (temp.value - 32f32) * (5f32 / 9f32),
                TempUnit::Celsius => temp.value,
                TempUnit::Kelvin => temp.value - 273.15_f32,
            };
            (temp.to_celsius().value - expected).abs() < std::f32::EPSILON
        }
        quickcheck(prop as fn(Temperature) -> bool);
    }

    #[test]
    fn test_to_fahrenheit_conversion() {
        fn prop(temp: Temperature) -> bool {
            let expected = match temp.unit {
                TempUnit::Fahrenheit => temp.value,
                TempUnit::Celsius => (temp.value * (9f32 / 5f32)) + 32f32,
                TempUnit::Kelvin => (temp.value - 273.15_f32) * (9f32 / 5f32) + 32f32,
            };
            (temp.to_fahrenheit().value - expected).abs() < std::f32::EPSILON
        }
        quickcheck(prop as fn(Temperature) -> bool);
    }
    #[test]
    fn test_to_kelvin_conversion() {
        fn prop(temp: Temperature) -> bool {
            let expected = match temp.unit {
                TempUnit::Fahrenheit => (temp.value - 32f32) * (5f32 / 9f32) + 273.15_f32,
                TempUnit::Celsius => temp.value + 273.15_f32,
                TempUnit::Kelvin => temp.value,
            };
            (temp.to_kelvin().value - expected).abs() < std::f32::EPSILON
        }
        quickcheck(prop as fn(Temperature) -> bool);
    }
}
