use day_03::{result_uncorrupt_file, result_uncorrupt_file_with_closures};

#[test]
fn it_should_uncorrupt_file() {
    assert_eq!(result_uncorrupt_file("tests/resources/puzzle.txt"), 161);
}

#[test]
fn it_should_uncorrupt_file_with_closures() {
    assert_eq!(result_uncorrupt_file_with_closures("tests/resources/puzzle_with_closures.txt"), 48);
}