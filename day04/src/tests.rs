use super::*;

#[test]
fn it_works() {
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
