pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }

    let upper_bound = upper_bound as usize;

    let size = upper_bound + 2; // +2 is to handle 0 and 1 at their indices
    let mut all: Vec<bool> = Vec::with_capacity(size);
    all.resize(size, true);

    // not primes
    all[0] = false;
    all[1] = false;

    let n = (upper_bound as f64).sqrt().floor() as usize;

    for i in 2..n + 1 {
        if all[i] {
            for j in (i * i..size).step_by(i) {
                all[j] = false;
            }
        }
    }

    (0..upper_bound + 1)
        .filter(|i| all[*i])
        .map(|i| i as u64)
        .collect()
}
