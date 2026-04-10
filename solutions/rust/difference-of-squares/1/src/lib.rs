
pub fn square_of_sum(n: u32) -> u32 {
    let sum:u32 = (1..=n).sum();
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // todo!("sum of squares of 1...{n}")
    let sum:u32 = (1..=n).map(|e| e.pow(2)).sum();
    sum
}

pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    let diff = dbg!(square_of_sum(n)) - dbg!(sum_of_squares(n));
    diff
}