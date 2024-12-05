use day_03::{result_uncorrupt_file, result_uncorrupt_file_with_closures};

fn main() {
    let result = result_uncorrupt_file("src/resources/puzzle.txt");

    // 160672468
    println!("Multiply result in uncorrupted file: {result}");

    let result = result_uncorrupt_file_with_closures("src/resources/puzzle_with_closures.txt");

    println!("Multiply result in uncorrupted file with closures: {result}");
}
