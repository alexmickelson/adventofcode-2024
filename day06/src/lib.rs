use std::fs;

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    // println!("{}", data);
    data
}

pub fn parse_map(input: &str) -> Vec<Vec<char>> {
    let lines = input
        .split("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    lines
}

pub fn trace_guards_path(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut local_grid: Vec<Vec<char>> = grid
        .iter()
        .map(|l| l.iter().cloned().collect::<Vec<char>>())
        .collect();
    while guard_is_in_grid(&local_grid) {
        local_grid = move_guard_once(&local_grid);
    }
    local_grid
}

pub fn move_guard_once(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut local_grid = grid.clone();
    for i in 0..local_grid.len() {
        for j in 0..local_grid[i].len() {
            if local_grid[i][j] == '^' {
                local_grid[i][j] = 'X';
                if i > 0 {
                    if local_grid[i - 1][j] != '#' {
                        local_grid[i - 1][j] = '^';
                    } else {
                        local_grid[i][j] = '>';
                    }
                }
                return local_grid;
            }
            if local_grid[i][j] == '>' {
                local_grid[i][j] = 'X';
                if j < (local_grid[i].len() - 1) {
                    if local_grid[i][j + 1] != '#' {
                        local_grid[i][j + 1] = '>';
                    } else {
                        local_grid[i][j] = 'v';
                    }
                }
                return local_grid;
            }
            if local_grid[i][j] == 'v' {
                local_grid[i][j] = 'X';
                if i < (local_grid.len() - 1) {
                    if local_grid[i + 1][j] != '#' {
                        local_grid[i + 1][j] = 'v';
                    } else {
                        local_grid[i][j] = '<';
                    }
                }
                return local_grid;
            }
            if local_grid[i][j] == '<' {
                local_grid[i][j] = 'X';
                if j > 0 {
                    if local_grid[i][j - 1] != '#' {
                        local_grid[i][j - 1] = '<';
                    } else {
                        local_grid[i][j] = '^';
                    }
                }
                return local_grid;
            }
        }
    }

    local_grid
}

fn guard_is_in_grid(grid: &Vec<Vec<char>>) -> bool {
    let guard_chars = ['^', '>', 'v', '<'];
    for row in grid {
        for col in row {
            if guard_chars.contains(col) {
                return true;
            }
        }
    }
    false
}

pub fn count_path(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .map(|l| l.iter().filter(|c| **c == 'X').count())
        .sum()
}

pub fn count_paths(grid: &Vec<Vec<char>>) -> usize {
    let mut path_grid = grid.clone();
    let mut guard_grid = grid.clone();
    let mut count = 0;
    while guard_is_in_grid(&guard_grid) {
        path_grid = mark_guard_directional_path(&path_grid, &guard_grid);
        guard_grid = move_guard_once(&guard_grid);
        print_grid(&guard_grid);
        println!();
        print_grid(&path_grid);

        if next_obstacle_is_cycle(&path_grid, &guard_grid) {
            count += 1;
            println!("count is {}", count);
        }
        else {
            println!("next spot not obstacle");
        }
        println!("\n-----------------------------\n");
    }
    print_grid(&path_grid);
    count
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c)
        }
        print!("\n")
    }
}

pub fn next_obstacle_is_cycle(local_grid: &Vec<Vec<char>>, guard_grid: &Vec<Vec<char>>) -> bool {
    for i in 0..guard_grid.len() {
        for j in 0..guard_grid[i].len() {
            if guard_grid[i][j] == '^' {
                if i > 0 {
                    if local_grid[i - 1][j] != '#' {
                        if local_grid[i][j + 1] == '-' || local_grid[i][j + 1] == '+' {
                            return true;
                        }
                    }
                }
                return false;
            }
            if guard_grid[i][j] == '>'  {
                if j < (local_grid[i].len() - 1) {
                    if local_grid[i][j + 1] != '#' {
                        if local_grid[i + 1][j] == '|' || local_grid[i + 1][j] == '+' {
                            return true;
                        }
                    }
                }
                return false;
            }
            if guard_grid[i][j] == 'v' {
                if i < (local_grid.len() - 1) {
                    if local_grid[i + 1][j] != '#' {
                        if local_grid[i][j - 1] == '-' || local_grid[i][j - 1] == '+' {
                            return true;
                        }
                    }
                }
                return false;
            }
            if guard_grid[i][j] == '<' {
                if j > 0 {
                    if local_grid[i][j - 1] != '#' {
                        if local_grid[i - 1][j] == '|' || local_grid[i - 1][j] == '+' {
                            return true;
                        }
                    }
                }
                return false;
            }
        }
    }

    false
}

fn mark_guard_directional_path(
    grid: &Vec<Vec<char>>,
    guard_grid: &Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    let mut path_grid = grid.clone();
    for i in 0..path_grid.len() {
        for j in 0..path_grid[i].len() {
            if guard_grid[i][j] == '^' {
                if path_grid[i][j] == '-' || path_grid[i][j] == '+' {
                    path_grid[i][j] = '+';
                } else {
                    path_grid[i][j] = '|';
                }
                return path_grid;
            }
            if guard_grid[i][j] == '>' {
                if path_grid[i][j] == '|' || path_grid[i][j] == '+' {
                    path_grid[i][j] = '+';
                } else {
                    path_grid[i][j] = '-';
                }

                return path_grid;
            }
            if guard_grid[i][j] == 'v' {
                if path_grid[i][j] == '-' || path_grid[i][j] == '+' {
                    path_grid[i][j] = '+';
                } else {
                    path_grid[i][j] = '|';
                }

                return path_grid;
            }
            if guard_grid[i][j] == '<' {
                if path_grid[i][j] == '|' || path_grid[i][j] == '+' {
                    path_grid[i][j] = '+';
                } else {
                    path_grid[i][j] = '-';
                }

                return path_grid;
            }
        }
    }

    path_grid
}

#[cfg(test)]
mod tests;
