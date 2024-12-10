use std::{collections::HashSet, fs};

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    data
}

pub fn count_antinodes(input: &str) -> i64 {
    let grid = make_grid(input);

    let char_coords = get_char_coords(&grid);
    let max_rows = grid.len() as i64;
    let max_col = grid[0].len() as i64;

    let antinodes = count_antinode_coords(&char_coords, &max_rows, &max_col);

    let unique_values: HashSet<_> = antinodes.iter().cloned().collect();

    // Count the number of unique values
    unique_values.len() as i64
}

fn count_antinode_coords(
    input: &Vec<((i64, i64), &char)>,
    row_len: &i64,
    col_len: &i64,
) -> Vec<(i64, i64)> {
    if input.len() == 1 {
        return vec![];
    }

    let point = input[0];
    let rest = input[1..].to_vec();

    // let mut count: i64 = 0;

    let mut antinodes: Vec<(i64, i64)> = vec![];
    for other_point in rest.iter() {
        let ((row1, col1), c1) = point;
        let ((row2, col2), c2) = other_point;

        if &c1 == c2 {
            let row_diff = row1 - row2;
            let col_diff = col1 - col2;

            let first_antinode = (row1 + row_diff, col1 + col_diff);
            let second_antinode = (row2 - row_diff, col2 - col_diff);

            if first_antinode.0 >= 0
                && &first_antinode.0 < row_len
                && first_antinode.1 >= 0
                && &first_antinode.1 < col_len
            {
                // println!(
                //     "antinode for ({}, {}) and ({}, {}) at ({}, {})",
                //     row1, col1, row2, col2, first_antinode.0, first_antinode.1
                // );
                antinodes.push(first_antinode);
            }
            if second_antinode.0 >= 0
                && &second_antinode.0 < row_len
                && second_antinode.1 >= 0
                && &second_antinode.1 < col_len
            {
                // println!(
                //     "antinode for ({}, {}) and ({}, {}) at ({}, {})",
                //     row1, col1, row2, col2, second_antinode.0, second_antinode.1
                // );
                antinodes.push(second_antinode);
            }
        }
    }

    let other_antinodes = count_antinode_coords(&rest, row_len, col_len);
    let all_antinodes = [antinodes, other_antinodes].concat();
    all_antinodes
}

fn get_char_coords(grid: &Vec<Vec<char>>) -> Vec<((i64, i64), &char)> {
    let coordinates: Vec<((i64, i64), &char)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_j, c)| c != &&'.')
                .map(|(j, c)| ((i as i64, j as i64), c))
                .collect::<Vec<((i64, i64), &char)>>()
        })
        .collect();
    coordinates
}

fn make_grid(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn count_resonant_antinodes(input: &str) -> i64 {
    let grid = make_grid(input);

    let char_coords = get_char_coords(&grid);
    let max_rows = grid.len() as i64;
    let max_col = grid[0].len() as i64;

    let antinodes = get_resonant_antinode_coords(&char_coords, &max_rows, &max_col);

    let unique_values: HashSet<(i64, i64)> = antinodes.iter().cloned().collect();
    let count: i64 = unique_values.len() as i64;
    let mut sorted_values = unique_values
        .into_iter()
        .collect::<Vec<(i64, i64)>>();
    sorted_values.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    println!("{:?}", sorted_values);

    count
}

fn get_resonant_antinode_coords(
    input: &Vec<((i64, i64), &char)>,
    row_len: &i64,
    col_len: &i64,
) -> Vec<(i64, i64)> {
    if input.len() == 1 {
        return vec![];
    }

    let point = input[0];
    let rest = input[1..].to_vec();

    let mut antinodes: Vec<(i64, i64)> = vec![];
    for other_point in rest.iter() {
        let ((row1, col1), c1) = point;
        let ((row2, col2), c2) = other_point;

        if &c1 == c2 {
            let row_diff = row1 - row2;
            let col_diff = col1 - col2;

            let mut antinode = (row1, col1);
            while antinode.0 >= 0
                && &antinode.0 < row_len
                && antinode.1 >= 0
                && &antinode.1 < col_len
            {
                antinodes.push(antinode);
                antinode = (antinode.0 + row_diff, antinode.1 + col_diff);
            }
            antinode = (row1, col1);
            while antinode.0 >= 0
                && &antinode.0 < row_len
                && antinode.1 >= 0
                && &antinode.1 < col_len
            {
                antinodes.push(antinode);
                antinode = (antinode.0 - row_diff, antinode.1 - col_diff);
            }
        }
    }

    let other_antinodes = get_resonant_antinode_coords(&rest, row_len, col_len);
    let all_antinodes = [antinodes, other_antinodes].concat();
    all_antinodes
}
#[cfg(test)]
mod tests;
