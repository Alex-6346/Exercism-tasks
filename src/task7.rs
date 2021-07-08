#[allow(dead_code)]
pub fn squareofsum_minus_sumofsquares(n: u32) -> u32 {
    let mut square_of_sum: u32 = 0;
    let mut sum_of_squares: u32 = 0;

    for i in 1..n + 1 {
        square_of_sum += i;
        sum_of_squares += i * i;
    }
    square_of_sum *= square_of_sum;
    square_of_sum - sum_of_squares
}
