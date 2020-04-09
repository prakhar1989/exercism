pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|x| is_multiple_of(*x, factors)).sum()
}

fn is_multiple_of(x: u32, factors: &[u32]) -> bool {
    factors.iter().map(|f| x % *f).any(|r| r == 0)
}
