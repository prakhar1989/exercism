pub fn raindrops(n: u32) -> String {
    if n % 4 == 0 {
        return String::from("Pling");
    }
    if n % 5 == 0 {
        return String::from("Plang");
    }
    if n % 7 == 0 {
        return String::from("Plong");
    }
    return n.to_string();
}
