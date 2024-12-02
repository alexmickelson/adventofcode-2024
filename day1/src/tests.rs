use super::*;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn can_read_file_from_string() {
    let result = read_file_from_path("test.txt");
    assert_eq!(result, "this is the string")
}
