mod utility;

mod day_1;

fn main() {
    day_1_part_2();
}

#[allow(dead_code)]
fn day_1_part_1() {
    let file = utility::read_from_resource("day_1_input.txt");
    let input = utility::file_lines_to_numbers(file);

    let result = day_1::measure_depth_increase(input);

    println!("Day 1 - Part 1: {}", result);
}

#[allow(dead_code)]
fn day_1_part_2() {
    let file = utility::read_from_resource("day_1_input.txt");
    let input = utility::file_lines_to_numbers(file);

    let result = day_1::measure_depth_increase_sliding(input);

    println!("Day 1 - Part 2: {}", result);
}