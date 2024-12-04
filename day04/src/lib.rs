use std::fs;

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    println!("{}", data);
    data
}

pub fn count_xmas(input: &str) -> usize {
    let values = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| l.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    println!("output is {}", values[0][1]);
    1
}

#[cfg(test)]
mod tests;
