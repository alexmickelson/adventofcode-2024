use super::*;

#[test]
fn can_parse_inputs() {
    let input = "1 2 3
4 5 6";
    let levels = parse_levels(input);
    assert_eq!(levels, vec![vec![1, 2, 3], vec![4, 5, 6]]);
}

#[test]
fn level_is_safe_1() {
    let level = vec![7, 6, 4, 2, 1];

    let is_safe = level_is_safe(&level);
    assert_eq!(is_safe, true);
}

#[test]
fn level_is_safe_2() {
    let level = vec![1, 2, 7, 8, 9];

    let is_safe = level_is_safe(&level);
    assert_eq!(is_safe, false);
}
#[test]
fn level_is_safe_3() {
    let level = vec![9, 7, 6, 2, 1];

    let is_safe = level_is_safe(&level);
    assert_eq!(is_safe, false);
}

#[test]
fn level_is_safe_4() {
    let level = vec![1, 3, 2, 4, 5];

    let is_safe = level_is_safe(&level);
    assert_eq!(is_safe, false);
}

#[test]
fn level_is_safe_5() {
    let level = vec![8, 6, 4, 4, 1];

    let is_safe = level_is_safe(&level);
    assert_eq!(is_safe, false);
}

#[test]
fn level_is_safe_6() {
    let level = vec![1, 3, 6, 7, 9];

    let is_safe = level_is_safe(&level);
    assert_eq!(is_safe, true);
}


// dampeners


#[test]
fn level_is_safe_1_with_dampener() {
    let level = vec![7, 6, 4, 2, 1];

    let is_safe = level_is_safe_with_dampener(&level);
    assert_eq!(is_safe, true);
}

#[test]
fn level_is_safe_2_with_dampener() {
    let level = vec![1, 2, 7, 8, 9];

    let is_safe = level_is_safe_with_dampener(&level);
    assert_eq!(is_safe, false);
}
#[test]
fn level_is_safe_3_with_dampener() {
    let level = vec![9, 7, 6, 2, 1];

    let is_safe = level_is_safe_with_dampener(&level);
    assert_eq!(is_safe, false);
}

#[test]
fn level_is_safe_4_with_dampener() {
    let level = vec![1, 3, 2, 4, 5];

    let is_safe = level_is_safe_with_dampener(&level);
    assert_eq!(is_safe, true);
}

#[test]
fn level_is_safe_5_with_dampener() {
    let level = vec![8, 6, 4, 4, 1];

    let is_safe = level_is_safe_with_dampener(&level);
    assert_eq!(is_safe, true);
}

#[test]
fn level_is_safe_6_with_dampener() {
    let level = vec![1, 3, 6, 7, 9];

    let is_safe = level_is_safe_with_dampener(&level);
    assert_eq!(is_safe, true);
}
