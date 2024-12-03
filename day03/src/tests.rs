use super::*;

#[test]
fn can_get_match_from_string() {
    let input = "mul(5,5)";
    let matches = get_all_matches(input);

    assert_eq!(matches, vec!["mul(5,5)"]);
}

#[test]
fn can_get_many_matches_from_string() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let matches = get_all_matches(input);

    assert_eq!(
        matches,
        vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]
    );
}

#[test]
fn get_enabled_matches_test() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let matches = get_enabled_matches(input);

    assert_eq!(
        matches,
        vec!["mul(2,4)", "mul(8,5)"]
    );
}
