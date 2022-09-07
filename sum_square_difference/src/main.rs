fn main() {
    // Find the difference between the sum of the squares of the first one hundred natural numbers
    // and the square of the sum.

    let square = |n: i32| -> i32 { n * n };

    // Sum of squares
    let sum_of_squares: i32 = (1..=100).into_iter()
        .map(square)
        .sum();

    // Square of sum
    let square_of_sum = square((1..=100).into_iter().sum());

    // Sum square difference
    let sum_square_difference = square_of_sum - sum_of_squares;

    println!("{}", sum_square_difference);
}
