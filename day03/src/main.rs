use day03::*;


// fn main() {
//   let input = read_file_from_path("./input.txt");
//   let muls = get_all_matches(&input);
//   let sum: usize = muls.iter().map(|m| evaluate_mul(m)).sum();
//   println!("sum is {}", sum);
// }

fn main() {
  let input = read_file_from_path("./input.txt");
  let muls = get_enabled_matches(&input);
  let sum: usize = muls.iter().map(|m: &&str| evaluate_mul(m)).sum();
  println!("sum is {}", sum);
}