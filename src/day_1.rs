/// Counts the number of times depth increases.
///
/// # Examples
///
/// ```
/// let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
/// let result = measure_depth_increase(measurements);
///
/// assert_eq!(result, 7);
/// ```
pub fn measure_depth_increase(measurements: Vec<i32>) -> i32 {
    let mut increase_count = -1;
    let mut last_number = -1;

    for measurement in measurements {
        if measurement > last_number {
            increase_count += 1;
        }

        last_number = measurement;
    }

    return increase_count;
}

/// Counts the number of depth increases within a 3-measurement sliding window.
///
/// # Examples
///
/// ```
/// let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
/// let result = measure_depth_increase_sliding(measurements);
///
/// assert_eq!(result, 5);
/// ```
pub fn measure_depth_increase_sliding(measurements: Vec<i32>) -> i32 {
    let mut increase_count = -1;
    let mut window: Vec<i32> = Vec::new();
    let mut last_window_sum = 0;

    for measurement in measurements {
        window.push(measurement);

        if window.len() < 3 {
            continue;
        } else if window.len() > 3 {
            window.remove(0);
        }

        let window_sum = window.iter().sum();

        if window_sum > last_window_sum {
            increase_count += 1;
        }

        last_window_sum = window_sum;
    }

    return increase_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_depth_increase() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = measure_depth_increase(measurements);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_measure_depth_increase_sliding() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = measure_depth_increase_sliding(measurements);

        assert_eq!(result, 5);
    }
}