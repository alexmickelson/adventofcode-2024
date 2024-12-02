use std::fs;

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    println!("{}", data);
    data
}

pub fn parse_levels(input: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = input.split("\n").collect();

    let levels: Vec<Vec<i32>> = lines
        .iter()
        .map(|&line| {
            line.split(" ")
                .into_iter()
                .map(|n| n.parse().expect("cannot parse number"))
                .collect()
        })
        .collect();
    levels
}

pub fn level_is_safe(input: &Vec<i32>) -> bool {
    let mut prev = input[0];

    let is_incrementing = is_incrementing(&input);

    for cur in input[1..].iter() {
        let delta = (prev - cur).abs();

        if delta < 1 || delta > 3 {
            return false;
        }

        if is_incrementing && prev > *cur {
            return false;
        }

        if !is_incrementing && *cur > prev {
            return false;
        }
        prev = *cur;
    }

    true
}

pub fn level_is_safe_with_dampener(input: &Vec<i32>) -> bool {
    let is_safe = level_is_safe(input);

    if is_safe {
        return true;
    }

    let length = input.len();

    let permutations: Vec<Vec<i32>> = (0..length)
        .map(|index| {
            let before = input[0..index].to_vec();
            let after = input[index + 1..].to_vec();

            [before, after].concat()
        })
        .collect();

    for perm in permutations {
        if level_is_safe(&perm) {
            return true;
        }
    }

    false
}

fn is_incrementing(input: &Vec<i32>) -> bool {
    let mut prev = input[0];

    for cur in input[1..].iter() {
        if &prev != cur {
            return &prev < cur;
        }
    }
    // all values are identical...
    false
}

#[cfg(test)]
mod tests;
