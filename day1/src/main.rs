use day1::*;

// fn main() {
//   let input = read_file_from_path("part1.txt");
//   let (first, second) = parse_two_lists(&input);
//   let sum = get_sums_of_each_list(&first, &second);
//   println!("the sum is: {}", sum);
// }
fn main() {
  let input = read_file_from_path("part1.txt");
  let (first, second) = parse_two_lists(&input);
  let sum = get_similarity_score(&first, &second);
  println!("the similarity is: {}", sum);
}
