use day05::*;

fn main() {
    let input = read_file_from_path("./input.txt");
    let rules = parse_rules(&input);
    let page_numbers = parse_update_page_numbers(&input);

    let correct_pages = page_numbers
        .iter()
        .filter(|p| !passes_rules(p.to_vec(), &rules))
        .map(|p| correct_bad_rule(p.to_vec(), &rules))
        .collect::<Vec<Vec<usize>>>();

    let middle_values = correct_pages
        .iter()
        .map(|l| {
            let length = l.len();
            let middle = length / 2;
            l[middle]
        })
        .collect::<Vec<usize>>();

    let sum: usize = middle_values.iter().sum();
    println!("sum is {}", sum);
}
