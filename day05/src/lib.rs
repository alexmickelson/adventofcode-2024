use std::fs;

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    println!("{}", data);
    data
}

pub fn parse_rules(input: &str) -> Vec<(usize, usize)> {
    let rules_part = input.split("\n\n").collect::<Vec<&str>>()[0];

    let rules = rules_part
        .split("\n")
        .map(|l| {
            let numbers = l
                .split("|")
                .map(|n| n.parse::<usize>().expect("cannot parse number in rule"))
                .collect::<Vec<usize>>();
            (numbers[0], numbers[1])
        })
        .collect::<Vec<(usize, usize)>>();

    return rules;
}

pub fn parse_update_page_numbers(input: &str) -> Vec<Vec<usize>> {
    let page_numbers_part = input.split("\n\n").collect::<Vec<&str>>()[1];

    let page_numbers = page_numbers_part
        .split("\n")
        .map(|l| {
            l.split(",")
                .map(|d| d.parse::<usize>().expect("cannot parse page number"))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    page_numbers
}

pub fn passes_rules(page_numbers: Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    for rule in rules {
        let (first, second) = rule;
        if page_numbers.contains(&first) && page_numbers.contains(&second) {
            let first_part_of_page_numbers = page_numbers
                .split(|v: &usize| v == second)
                .collect::<Vec<&[usize]>>()[0];
            if !first_part_of_page_numbers.contains(&first) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests;
