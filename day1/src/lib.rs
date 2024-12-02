use std::fs;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    println!("{}", data);
    data
}
pub fn parse_two_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();

    for line in input.split("\n") {
        let parts: Vec<&str> = line.split("   ").collect();
        let part1 = parts[0].parse::<i32>().expect("unable to parse part 1");
        let part2 = parts[1].parse::<i32>().expect("unable to parse part 1");
        first.push(part1);
        second.push(part2);
    }

    (first, second)
}

pub fn get_sums_of_each_list(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    let mut first_sorted: Vec<i32> = first.clone();
    let mut second_sorted: Vec<i32> = second.clone();

    first_sorted.sort();
    second_sorted.sort();

    // println!("first {:?}", first_sorted);
    // println!("second {:?}", second_sorted);

    let mut sum = 0;
    for i in 0..first_sorted.len() {
        let local: i32 = (first_sorted[i] - second_sorted[i]).abs();
        sum += local;
    }
    sum
}
pub fn get_similarity_score(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    let mut total = 0;

    for i in first {
        let count_in_second: i32 = second.iter().filter(|&n| n == i).count().try_into().expect("could not convert sum to int");
        total += i * count_in_second;
    }
    total
}

#[cfg(test)]
mod tests;
