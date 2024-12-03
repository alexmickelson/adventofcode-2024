use regex::Regex;
use std::fs;

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    println!("{}", data);
    data
}

pub fn get_all_matches(input: &str) -> Vec<&str> {
    let pattern = r"mul\([0-9]{1,3},[0-9]{1,3}\)";
    let re = Regex::new(pattern).expect("could not create regex pattern");

    let matches: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    matches
}

pub fn get_enabled_matches(input: &str) -> Vec<&str> {
    let first_section = input.split("don't()").collect::<Vec<&str>>()[0];

    let start_disabled_sections = input.split("don't()").collect::<Vec<&str>>()[1..].to_vec();

    let mut enabled_sections: Vec<&str> = start_disabled_sections
        .iter()
        .flat_map(|s| s.split("do()").skip(1))
        .collect();

    enabled_sections.insert(0, first_section);

    let matches = enabled_sections
        .iter()
        .flat_map(|s| get_all_matches(s))
        .collect();

    matches
}

pub fn evaluate_mul(input: &str) -> usize {
    let part1: Vec<&str> = input.split("(").collect();
    let part2: Vec<&str> = part1[1].split(")").collect();
    let part3: Vec<&str> = part2[0].split(",").collect();

    let first: usize = part3[0]
        .parse()
        .expect("could not parse first number in mul");
    let second: usize = part3[1]
        .parse()
        .expect("could not parse second number in mul");
    first * second
}

#[cfg(test)]
mod tests;
