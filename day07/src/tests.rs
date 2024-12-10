use super::*;

#[test]
fn can_parse_equation() {
    let input = "190: 10 19";
    let (result, numbers) = parse_equation(&input);

    assert_eq!(result, 190);
    assert_eq!(numbers, vec![10, 19]);
}

#[test]
fn input_1() {
    let input = "190: 10 19";
    let (result, numbers) = parse_equation(&input);

    assert_eq!(
        can_be_corrected_with_add_and_multiply(&result, &numbers),
        true
    );
}

#[test]
fn input_2() {
    let input = "3267: 81 40 27";
    let (result, numbers) = parse_equation(&input);

    assert_eq!(
        can_be_corrected_with_add_and_multiply(&result, &numbers),
        true
    );
}

#[test]
fn other_inputs() {
    let cases = [
        ("190: 10 19", true),
        ("3267: 81 40 27", true),
        ("83: 17 5", false),
        ("156: 15 6", false),
        ("7290: 6 8 6 15", false),
        ("161011: 16 10 13", false),
        ("192: 17 8 14", false),
        ("21037: 9 7 18 13", false),
        ("292: 11 6 16 20", true),
    ];
    for (input, expected) in cases {
        let (result, numbers) = parse_equation(input);

        assert_eq!(
            can_be_corrected_with_add_and_multiply(&result, &numbers),
            expected
        );
    }
}

#[test]
fn part_2_inputs() {
    let cases = [
        ("190: 10 19", true),
        ("3267: 81 40 27", true),
        ("83: 17 5", false),
        ("156: 15 6", true),
        ("7290: 6 8 6 15", true),
        ("161011: 16 10 13", false),
        ("192: 17 8 14", true),
        ("21037: 9 7 18 13", false),
        ("292: 11 6 16 20", true),
    ];
    for (input, expected) in cases {
        let (result, numbers) = parse_equation(input);

        assert_eq!(
            can_be_corrected_with_add_and_multiply_and_concat(&result, &numbers),
            expected,
            "Test failed for input: {}",
            input
        );
    }
}
