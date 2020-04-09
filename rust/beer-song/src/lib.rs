pub fn verse(n: u32) -> String {
    if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    if n == 1 {
        return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }
    return format!( "{} of beer on the wall, {} of beer.\nTake one down and pass it around, {} of beer on the wall.\n", pluralize(n), pluralize(n), pluralize(n - 1));
}

fn pluralize(n: u32) -> String {
    if n == 1 {
        format!("1 bottle")
    } else {
        format!("{} bottles", n)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..start + 1)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
