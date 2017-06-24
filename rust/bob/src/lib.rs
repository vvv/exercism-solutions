pub fn reply(phrase: &str) -> &str {
    if phrase.trim().is_empty() {
        "Fine. Be that way!"
    } else if phrase.ends_with("?") {
        "Sure."
    } else {
        let s = phrase.trim_matches(|c: char| !c.is_alphabetic());
        if !s.is_empty() && s.to_uppercase() == s {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    }
}
