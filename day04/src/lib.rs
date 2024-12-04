use std::fs;

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    println!("{}", data);
    data
}

#[cfg(test)]
mod tests;
