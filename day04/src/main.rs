use day04::*;

fn main() {
  let input = read_file_from_path("./input.txt");
  let count = count_x_mas(&input);
  println!("count is: {}", count)
}
