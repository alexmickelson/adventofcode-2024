use std::{borrow::Borrow, fs};

pub fn read_file_from_path(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    println!("{}", data);
    data
}

pub fn parse_equation(input: &str) -> (usize, Vec<usize>) {
    let parts = input.split(": ").collect::<Vec<&str>>();

    let total = parts[0]
        .parse::<usize>()
        .expect("cannot parse total of input");
    let rest = parts[1]
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("cannot parse number in rest"))
        .collect::<Vec<usize>>();
    (total, rest)
}

pub fn can_be_corrected_with_add_and_multiply(total: &usize, equation: &Vec<usize>) -> bool {
    if equation.len() == 2 {
        if &(equation[0] + equation[1]) == total {
            return true;
        }
        if &(equation[0] * equation[1]) == total {
            return true;
        }
        return false;
    }

    let sum_first_part = equation[0] + equation[1];
    let new_equation = [vec![sum_first_part], equation[2..].to_vec()].concat();

    if can_be_corrected_with_add_and_multiply(total, &new_equation) {
        return true;
    }
    let mult_first_part = equation[0] * equation[1];
    let new_mult_equation = [vec![mult_first_part], equation[2..].to_vec()].concat();

    if can_be_corrected_with_add_and_multiply(total, &new_mult_equation) {
        return true;
    }

    false
}
pub fn can_be_corrected_with_add_and_multiply_and_concat(
    total: &usize,
    equation: &Vec<usize>,
) -> bool {
    if equation.len() == 2 {
        if &(equation[0] + equation[1]) == total {
            return true;
        }
        if &(equation[0] * equation[1]) == total {
            return true;
        }

        if &format!("{}{}", equation[0], equation[1])
            .parse::<usize>()
            .expect("cannot parse concat total")
            == total
        {
            return true;
        }
        return false;
    }

    let sum_first_part = equation[0] + equation[1];
    let new_equation = [vec![sum_first_part], equation[2..].to_vec()].concat();

    if can_be_corrected_with_add_and_multiply_and_concat(total, &new_equation) {
        return true;
    }
    let mult_first_part = equation[0] * equation[1];
    let new_mult_equation = [vec![mult_first_part], equation[2..].to_vec()].concat();
    if can_be_corrected_with_add_and_multiply_and_concat(total, &new_mult_equation) {
        return true;
    }

    let concat_first_part = format!("{}{}", equation[0], equation[1])
        .parse::<usize>()
        .expect("cannot parse concat total second instance");
    let new_concat_equation = [vec![concat_first_part], equation[2..].to_vec()].concat();
    if can_be_corrected_with_add_and_multiply_and_concat(total, &new_concat_equation) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests;
