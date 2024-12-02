use day02::*;

// fn main() {
//   let input = read_file_from_path("part1.txt");
//   let (first, second) = parse_two_lists(&input);
//   let sum = get_sums_of_each_list(&first, &second);
//   println!("the sum is: {}", sum);
// }
fn main() {
    let input = read_file_from_path("./input.txt");
    let levels = parse_levels(&input);

    let safe_levels = levels
      .iter()
      .filter(|&l| level_is_safe(l.to_vec()))
      .count();
    println!("safe levels: {}", safe_levels)
}
