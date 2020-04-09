pub fn reply(mut message: &str) -> &str {
    message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_yelling =
        message.contains(char::is_alphabetic) && message == message.to_ascii_uppercase();

    if message.ends_with("?") {
        if is_yelling {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }

    if is_yelling {
        return "Whoa, chill out!";
    }

    return "Whatever.";
}
