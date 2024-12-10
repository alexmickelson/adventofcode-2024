use day07::*;

// fn main() {
//   let input = read_file_from_path("part1.txt");
//   let (first, second) = parse_two_lists(&input);
//   let sum = get_sums_of_each_list(&first, &second);
//   println!("the sum is: {}", sum);
// }
fn main() {
    let input = read_file_from_path("input.txt");
    let working_equations = input
        .split("\n")
        .map(|l| parse_equation(l))
        .filter(|(total, equation)| can_be_corrected_with_add_and_multiply_and_concat(total, equation))
        .collect::<Vec<(usize, Vec<usize>)>>();
    let sum: usize = working_equations
        .iter()
        .map(|(total, _equation)| total)
        .sum();
    println!("sum is {}", sum);
}
