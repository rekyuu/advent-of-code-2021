use std::env;
use std::fs;

/// Reads a file from the `resources/` directory as a string.
///
/// # Examples
///
/// ```
/// let file = read_from_resource("test.txt");
///
/// assert_eq!(file, "1\n2\n3");
/// ```
pub fn read_from_resource(filename: &str) -> String {
    let filepath = format!("{}/{}/{}", env!("CARGO_MANIFEST_DIR"), "resources", filename);
    let data = fs::read_to_string(filepath)
        .expect("Unable to read file!");

    return data;
}

/// Reads a string input as an array of numbers.
///
/// # Examples
///
/// ```
/// let data = String::from("1\n2\n3");
/// let result = file_lines_to_numbers(data);
///
/// assert_eq!(result, vec![1, 2, 3]);
/// ```
pub fn file_lines_to_numbers(input: String) -> Vec<i32> {
    let lines = input.lines();
    let numbers = lines
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    return numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from_resource() {
        let file = read_from_resource("test.txt");

        assert_eq!(file, "1\n2\n3");
    }

    #[test]
    fn test_file_lines_to_numbers() {
        let data = String::from("1\n2\n3");
        let result = file_lines_to_numbers(data);

        assert_eq!(result, vec![1, 2, 3]);
    }
}