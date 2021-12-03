/// Calculates power consumption based on a constructed byte array based on
/// the most and least common bits in each position.
///
/// For Gamma, this is considered the most common bits in each position.
/// For Epsilon, this is considered the least.
///
/// # Examples
/// ```
/// let diagnostics = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
///     .iter()
///     .map(|&s| s.into())
///     .collect();
///
/// assert_eq!(calculate_power_consumption(diagnostics), 198);
/// ```
pub fn calculate_power_consumption(diagnostics: Vec<String>) -> i32 {
    let bytes_length = diagnostics[0].chars().count();

    let mut gamma_vec: Vec<char> = Vec::new();
    let mut epsilon_vec: Vec<char> = Vec::new();
    let mut count = 0;

    while count < bytes_length {
        let most_common_bit = determine_most_common_bit(&diagnostics, count);

        if most_common_bit == '1' {
            gamma_vec.push('1');
            epsilon_vec.push('0');
        } else {
            gamma_vec.push('0');
            epsilon_vec.push('1');
        }

        count += 1;
    }

    let gamma = binary_vector_to_number(gamma_vec);
    let epsilon = binary_vector_to_number(epsilon_vec);

    return gamma * epsilon;
}

/// Calculates life support rating based on the most and least common bits in each position.
/// Byte entries are removed from possible results for each position.
///
/// For Oxygen, this is considered the most common bits in each position.
/// For CO2, this is considered the least.
///
/// # Examples
///
/// ```
/// let diagnostics = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
///     .iter()
///     .map(|&s| s.into())
///     .collect();
///
/// assert_eq!(calculate_life_support_rating(diagnostics), 230);
/// ```
pub fn calculate_life_support_rating(diagnostics: Vec<String>) -> i32 {
    let bytes_length = diagnostics[0].chars().count();

    let mut oxygen_vec: Vec<char> = Vec::new();
    let mut co2_vec: Vec<char> = Vec::new();
    let mut possible_oxygen_ratings = diagnostics.to_vec();
    let mut possible_co2_ratings = diagnostics.to_vec();
    let mut count = 0;

    while count < bytes_length {
        let most_common_oxygen_bit = determine_most_common_bit(&possible_oxygen_ratings, count);
        let most_common_co2_bit = determine_most_common_bit(&possible_co2_ratings, count);

        oxygen_vec.push(most_common_oxygen_bit);

        if most_common_co2_bit == '1' {
            co2_vec.push('0');
        } else {
            co2_vec.push('1');
        }

        if possible_oxygen_ratings.iter().count() != 1 {
            possible_oxygen_ratings
                .retain(|x| x.starts_with(&String::from_iter(&oxygen_vec)));
        }

        if possible_co2_ratings.iter().count() != 1 {
            possible_co2_ratings
                .retain(|x| x.starts_with(&String::from_iter(&co2_vec)));
        }

        if possible_oxygen_ratings.iter().count() == 1 && possible_co2_ratings.iter().count() == 1 {
            break;
        }

        count += 1;
    }

    let oxygen = i32::from_str_radix(possible_oxygen_ratings.first().unwrap().as_str(), 2).unwrap();
    let co2 = i32::from_str_radix(possible_co2_ratings.first().unwrap().as_str(), 2).unwrap();

    return oxygen * co2;
}

/// Determines the most common bit in byte array based on position.
///
/// # Examples
///
/// ```
/// let diagnostics = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
///     .iter()
///     .map(|&s| s.into())
///     .collect();
///
/// assert_eq!(determine_most_common_bit(&diagnostics, 0), '1');
/// assert_eq!(determine_most_common_bit(&diagnostics, 1), '0');
/// assert_eq!(determine_most_common_bit(&diagnostics, 2), '1');
/// assert_eq!(determine_most_common_bit(&diagnostics, 3), '1');
/// assert_eq!(determine_most_common_bit(&diagnostics, 4), '0');
/// ```
fn determine_most_common_bit(diagnostics: &Vec<String>, position: usize) -> char {
    let mut ones = 0;
    let mut zeroes = 0;

    for line in diagnostics {
        let bit = line.chars().nth(position).unwrap();

        if bit == '1' {
            ones += 1;
        } else if bit == '0' {
            zeroes += 1;
        }
    }

    return if ones >= zeroes {
        '1'
    } else {
        '0'
    }
}

/// Converts a vector of characters representing bits to a number.
///
/// # Examples
///
/// ```
/// let input = vec!['1', '0', '1', '1', '0']
///     .iter()
///     .map(|&s| s.into())
///     .collect();
///
/// assert_eq!(binary_vector_to_number(input), 22);
/// ```
fn binary_vector_to_number(input: Vec<char>) -> i32 {
    let input_str = String::from_iter(input);
    return i32::from_str_radix(input_str.as_str(), 2).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_power_consumption() {
        let diagnostics = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(calculate_power_consumption(diagnostics), 198);
    }

    #[test]
    fn test_calculate_life_support_rating() {
        let diagnostics = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(calculate_life_support_rating(diagnostics), 230);
    }

    #[test]
    fn test_determine_most_common_bit() {
        let diagnostics = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(determine_most_common_bit(&diagnostics, 0), '1');
        assert_eq!(determine_most_common_bit(&diagnostics, 1), '0');
        assert_eq!(determine_most_common_bit(&diagnostics, 2), '1');
        assert_eq!(determine_most_common_bit(&diagnostics, 3), '1');
        assert_eq!(determine_most_common_bit(&diagnostics, 4), '0');
    }

    #[test]
    fn test_binary_vector_to_number() {
        let input = vec!['1', '0', '1', '1', '0']
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(binary_vector_to_number(input), 22);
    }
}