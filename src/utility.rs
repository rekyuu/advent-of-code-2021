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

/// Splits a string into a vector based on their lines..
///
/// # Examples
///
/// ```
/// let data = String::from("1\n2\n3");
/// let result = file_lines_to_vector(data);
///
/// assert_eq!(result, vec!["1", "2", "3"]);
/// ```
pub fn file_lines_to_vector(input: String) -> Vec<String> {
    return input.lines()
        .map(String::from)
        .collect();
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
    fn test_file_lines_to_vector() {
        let data = String::from("1\n2\n3");
        let result = file_lines_to_vector(data);

        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_file_lines_to_numbers() {
        let data = String::from("1\n2\n3");
        let result = file_lines_to_numbers(data);

        assert_eq!(result, vec![1, 2, 3]);
    }
}