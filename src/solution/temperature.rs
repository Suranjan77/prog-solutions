// Implement a program that converts temperatures between Celsius and Fahrenheit.

fn to_celcius(t: f32) -> f32 {
    (t - 32_f32) * (5_f32 / 9_f32)
}

fn to_fahrenheit(t: f32) -> f32 {
    t * (9_f32 / 5_f32) + 32_f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_celcius() {
        assert_eq!(8.888889_f32, to_celcius(48_f32));
    }

    #[test]
    fn test_to_fahrenheit() {
        assert_eq!(32_f32, to_fahrenheit(0_f32));
    }
}
