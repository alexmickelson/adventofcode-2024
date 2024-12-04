use super::*;

#[test]
fn part_1() {
  let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
  let matches = count_xmas(input);
  assert_eq!(matches, 18);
}

#[test]
fn part_2() {
  let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
  let matches = count_x_mas(input);
  assert_eq!(matches, 9);
}
