use std::{fmt::format, fs};

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    println!("{}", data);
    data
}

pub fn count_xmas(input: &str) -> usize {
    let values = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;

    for i in 0..values.len() {
        for j in 0..values[i].len() {
            if values[i][j] == 'X' {
                count += count_xmas_at_coordinate(&i, &j, &values);
            }
        }
    }
    count
}

fn count_xmas_at_coordinate(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    count += check_up(x, y, values);
    count += check_down(x, y, values);
    count += check_left(x, y, values);
    count += check_right(x, y, values);
    count += check_top_left(x, y, values);
    count += check_top_right(x, y, values);
    count += check_bottom_left(x, y, values);
    count += check_bottom_right(x, y, values);
    count
}

fn check_up(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    if *y < 3 {
        return 0;
    }
    let value = format!(
        "{}{}{}{}",
        values[*x][*y],
        values[*x][*y - 1],
        values[*x][*y - 2],
        values[*x][*y - 3]
    );
    if value == "XMAS" {
        return 1;
    }
    0
}

fn check_down(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    if *y > values.len() - 4 {
        return 0;
    }
    let value = format!(
        "{}{}{}{}",
        values[*x][*y],
        values[*x][*y + 1],
        values[*x][*y + 2],
        values[*x][*y + 3]
    );
    if value == "XMAS" {
        return 1;
    }
    0
}

fn check_left(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    if *x < 3 {
        return 0;
    }
    let value = format!(
        "{}{}{}{}",
        values[*x][*y],
        values[*x - 1][*y],
        values[*x - 2][*y],
        values[*x - 3][*y]
    );
    if value == "XMAS" {
        return 1;
    }
    0
}

fn check_right(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    if *x > values[0].len() - 4 {
        return 0;
    }
    let value = format!(
        "{}{}{}{}",
        values[*x][*y],
        values[*x + 1][*y],
        values[*x + 2][*y],
        values[*x + 3][*y]
    );
    if value == "XMAS" {
        return 1;
    }
    0
}
fn check_top_left(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    if *x < 3 {
        return 0;
    }
    if *y < 3 {
        return 0;
    }
    let value = format!(
        "{}{}{}{}",
        values[*x][*y],
        values[*x - 1][*y - 1],
        values[*x - 2][*y - 2],
        values[*x - 3][*y - 3]
    );
    if value == "XMAS" {
        return 1;
    }
    0
}
fn check_top_right(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    if *x > values[0].len() - 4 {
        return 0;
    }
    if *y < 3 {
        return 0;
    }
    let value = format!(
        "{}{}{}{}",
        values[*x][*y],
        values[*x + 1][*y - 1],
        values[*x + 2][*y - 2],
        values[*x + 3][*y - 3]
    );
    if value == "XMAS" {
        return 1;
    }
    0
}
fn check_bottom_left(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    if *x < 3 {
        return 0;
    }
    if *y > values.len() - 4 {
        return 0;
    }
    let value = format!(
        "{}{}{}{}",
        values[*x][*y],
        values[*x - 1][*y + 1],
        values[*x - 2][*y + 2],
        values[*x - 3][*y + 3]
    );
    if value == "XMAS" {
        return 1;
    }
    0
}
fn check_bottom_right(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    if *x > values[0].len() - 4 {
        return 0;
    }
    if *y > values.len() - 4 {
        return 0;
    }
    let value = format!(
        "{}{}{}{}",
        values[*x][*y],
        values[*x + 1][*y + 1],
        values[*x + 2][*y + 2],
        values[*x + 3][*y + 3]
    );
    if value == "XMAS" {
        return 1;
    }
    0
}

pub fn count_x_mas(input: &str) -> usize {
    let values = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;

    for i in 0..values.len() {
        for j in 0..values[i].len() {
            if values[i][j] == 'A' {
                count += count_x_mas_at_coordinate(&i, &j, &values);
            }
        }
    }
    count
}

fn count_x_mas_at_coordinate(x: &usize, y: &usize, values: &Vec<Vec<char>>) -> usize {
    if *x > values[0].len() - 2 {
        return 0;
    }
    if *y > values.len() - 2 {
        return 0;
    }
    if *x == 0 {
        return 0;
    }
    if *y == 0 {
        return 0;
    }

    let first_diag = format!("{}{}", values[*x - 1][*y - 1], values[*x + 1][*y + 1]);
    let second_diag = format!("{}{}", values[*x + 1][*y - 1], values[*x - 1][*y + 1]);

    if (first_diag == "MS" || first_diag == "SM") && (second_diag == "MS" || second_diag == "SM") {
        return 1;
    }

    0
}

#[cfg(test)]
mod tests;
