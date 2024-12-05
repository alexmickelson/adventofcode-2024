use std::vec;

use super::*;

#[test]
fn can_parse_rules() {
    let input = "47|53
97|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let expected = vec![(47, 53), (97, 13)];
    let actual = parse_rules(input);
    assert_eq!(actual, expected);
}

#[test]
fn can_parse_page_numbers() {
    let input = "47|53
97|13

75,47
97,61";
    let expected = vec![vec![75, 47], vec![97, 61]];
    let actual = parse_update_page_numbers(input);
    assert_eq!(actual, expected);
}

#[test]
fn can_pass_rules_1() {
    let rules = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    let line = vec![75, 47, 61, 53, 29];
    let passes = passes_rules(line, &rules);
    assert_eq!(passes, true);
}

#[test]
fn can_pass_rules_2() {
    let rules = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    let line = vec![97,61,53,29,13];
    let passes = passes_rules(line, &rules);
    assert_eq!(passes, true);
}
#[test]
fn can_pass_rules_3() {
    let rules = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    let line = vec![75,29,13];
    let passes = passes_rules(line, &rules);
    assert_eq!(passes, true);
}

#[test]
fn can_pass_rules_4() {
    let rules = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    let line = vec![75,97,47,61,53];
    let passes = passes_rules(line, &rules);
    assert_eq!(passes, false);
}
#[test]
fn can_pass_rules_5() {
    let rules = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    let line = vec![61,13,29];
    let passes = passes_rules(line, &rules);
    assert_eq!(passes, false);
}
#[test]
fn can_pass_rules_6() {
    let rules = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    let line = vec![97,13,75,29,47];
    let passes = passes_rules(line, &rules);
    assert_eq!(passes, false);
}


#[test]
fn can_fix_rule() {
    let rules = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    let line = vec![97,13,75,29,47];
    let fixed = correct_bad_rule(line, &rules);
    let passes = passes_rules(fixed, &rules);
    assert_eq!(passes, true);
}

#[test]
fn can_fix_rule_1() {
    let rules = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    let line = vec![75,97,47,61,53];
    let fixed = correct_bad_rule(line, &rules);
    assert_eq!(fixed, vec![97,75,47,61,53]);
}
#[test]
fn can_fix_rule_2() {
    let rules = vec![
        (47, 53),
        (97, 13),
        (97, 61),
        (97, 47),
        (75, 29),
        (61, 13),
        (75, 53),
        (29, 13),
        (97, 29),
        (53, 29),
        (61, 53),
        (97, 53),
        (61, 29),
        (47, 13),
        (75, 47),
        (97, 75),
        (47, 61),
        (75, 61),
        (47, 29),
        (75, 13),
        (53, 13),
    ];
    let line = vec![61,13,29];
    let fixed = correct_bad_rule(line, &rules);
    assert_eq!(fixed, vec![61,29,13]);
}