use day06::*;

// fn main() {
//   let input = read_file_from_path("part1.txt");
//   let (first, second) = parse_two_lists(&input);
//   let sum = get_sums_of_each_list(&first, &second);
//   println!("the sum is: {}", sum);
// }
fn main() {
    let input = read_file_from_path("./input.txt");
    let grid = parse_map(&input);

    let result = trace_guards_path(&grid);

    let count = count_path(&result);
    println!("count is {}", count)
}
