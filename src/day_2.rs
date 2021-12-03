/// Measures the distance based on depth and horizontal movement.
/// Result is base don horizontal position multiplied by depth.
///
/// # Examples
///
/// ```
/// let measurements = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
///     .iter()
///     .map(|&s| s.into())
///     .collect();
///
/// assert_eq!(measure_position(measurements), 150);
/// ```
pub fn measure_position(measurements: Vec<String>) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for measurement in measurements {
        let measurement_specs: Vec<&str> = measurement.split(" ").collect();

        let direction = measurement_specs[0];
        let amount = measurement_specs[1].parse::<i32>().unwrap();

        if direction == "forward" {
            horizontal_pos += amount;
        } else if direction == "up" {
            depth -= amount;
        } else if direction == "down" {
            depth += amount;
        }
    }

    return horizontal_pos * depth;
}

/// Measures the distance based on depth and horizontal movement.
/// Depth is determined based on aim and horizontal movement.
/// Result is base don horizontal position multiplied by depth.
///
/// # Examples
///
/// ```
/// let measurements = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
///     .iter()
///     .map(|&s| s.into())
///     .collect();
///
/// assert_eq!(measure_position_with_aim(measurements), 900);
/// ```
pub fn measure_position_with_aim(measurements: Vec<String>) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for measurement in measurements {
        let measurement_specs: Vec<&str> = measurement.split(" ").collect();

        let direction = measurement_specs[0];
        let amount = measurement_specs[1].parse::<i32>().unwrap();

        if direction == "forward" {
            horizontal_pos += amount;
            depth += aim * amount;
        } else if direction == "up" {
            aim -= amount;
        } else if direction == "down" {
            aim += amount;
        }
    }

    return horizontal_pos * depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_position() {
        let measurements = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(measure_position(measurements), 150);
    }

    #[test]
    fn test_measure_position_with_aim() {
        let measurements = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(measure_position_with_aim(measurements), 900);
    }
}