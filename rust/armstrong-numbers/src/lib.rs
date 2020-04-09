pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    let l = digits.len();
    digits.iter().map(|x| x.pow(l as u32)).sum::<u32>() == num
}

fn get_digits(n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut num = n;
    while num > 0 {
        let r = num % 10;
        digits.push(r);
        num = num / 10;
    }
    digits
}
