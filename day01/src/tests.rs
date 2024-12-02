use super::*;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn can_read_file_from_string() {
    let result = read_file_from_path("test.txt");
    assert_eq!(
        result,
        "3   4
4   3
2   5
1   3
3   9
3   3"
    )
}

#[test]
fn can_parse_two_lists() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let (first, second) = parse_two_lists(input);

    assert_eq!(first, vec![3, 4, 2, 1, 3, 3]);
    assert_eq!(second, vec![4, 3, 5, 3, 9, 3]);
}

#[test]
fn can_sum_smallest_numbers() {
    let first = vec![3, 4, 2, 1, 3, 3];
    let second = vec![4, 3, 5, 3, 9, 3];

    let sum = get_sums_of_each_list(&first, &second);

    assert_eq!(sum, 11);
}

#[test]
fn can_calculate_similarity_scores() {
    let first = vec![3, 4, 2, 1, 3, 3];
    let second = vec![4, 3, 5, 3, 9, 3];

    let sum = get_similarity_score(&first, &second);

    assert_eq!(sum, 31);
}