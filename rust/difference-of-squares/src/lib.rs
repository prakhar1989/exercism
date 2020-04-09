pub fn square_of_sum(n: u32) -> u32 {
    let x: u32 = (1..n + 1).sum();
    x * x
}

pub fn sum_of_squares(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        (1..n).map(|x| x * x).sum()
    }
}

pub fn difference(n: u32) -> u32 {
    sum_of_squares(n) - square_of_sum(n)
}
