#![allow(dead_code)]

/// Convert degrees Fahrenheit to degrees Celsius.
/// Formula - `(33.8°F − 32) × 5/9 = 1°C`
pub fn from_f_to_c(f: f32) -> f32 {
    (f - 32f32) * (5f32 / 9f32)
}

/// Convert Kelvin to degrees Celsius.
/// Formula - `0K − 273.15 = -273.1°C`
pub fn from_k_to_c(k: f32) -> f32 {
    k - 273.15_f32
}

/// Convert degrees Celsius to degrees Fahrenheit.
/// Formula - `(33.8°F − 32) × 5/9 = 1°C`
pub fn from_c_to_f(c: f32) -> f32 {
    c * (9f32 / 5f32) + 32f32
}

/// Convert Kelvin to degrees Fahrenheit.
/// Formula - `0K − 273.15 = -273.1°C`
/// Formula - `(33.8°F − 32) × 5/9 = 1°C`
pub fn from_k_to_f(k: f32) -> f32 {
    (k - 273.15_f32) * (9f32 / 5f32) + 32f32
}

/// Convert degrees Celsius to Kelvin.
/// Formula - `0K − 273.15 = -273.1°C`
pub fn from_c_to_k(c: f32) -> f32 {
    c + 273.15_f32
}

/// Convert degrees Fahrenheit to Kelvin.
/// Formula - `0K − 273.15 = -273.1°C`
/// Formula - `(33.8°F − 32) × 5/9 = 1°C`
pub fn from_f_to_k(f: f32) -> f32 {
    (f - 32f32) * (5f32 / 9f32) + 273.15_f32
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_f_to_c() {
        assert_eq!(from_f_to_c(32f32).round(), 0f32);
        assert_eq!(from_f_to_c(33.8f32).round(), 1f32);
        assert_eq!(from_f_to_c(50f32).round(), 10f32);
    }
    #[test]
    fn it_k_to_c() {
        assert_eq!(from_k_to_c(273.15_f32), 0f32);
        assert_eq!(from_k_to_c(274.15_f32), 1f32);
        assert_eq!(from_k_to_c(283.15_f32), 10f32);
    }
    #[test]
    fn it_c_to_k() {
        assert_eq!(from_c_to_k(0f32), 273.15_f32);
        assert_eq!(from_c_to_k(1f32), 274.15_f32);
        assert_eq!(from_c_to_k(10f32), 283.15_f32);
    }
    #[test]
    fn it_f_to_k() {
        assert_eq!(from_f_to_k(32f32), 273.15_f32);
        assert_eq!(from_f_to_k(33.8f32), 274.15_f32);
        assert_eq!(from_f_to_k(0f32), 255.372_22_f32);
        assert_eq!(from_f_to_k(10f32), 260.927_76_f32);
    }
    #[test]
    fn it_c_to_f() {
        assert_eq!(from_c_to_f(0f32).round(), 32f32);
        assert_eq!(from_c_to_f(1f32).round(), 34f32);
        assert_eq!(from_c_to_f(10f32).round(), 50f32);
    }
    #[test]
    fn it_k_to_f() {
        assert_eq!(from_k_to_f(273.15_f32).round(), 32f32);
        assert_eq!(from_k_to_f(274.15_f32).round(), 34f32);
        assert_eq!(from_k_to_f(255.372_22_f32).round(), 0f32);
        assert_eq!(from_k_to_f(260.927_76_f32).round(), 10f32);
    }
}
