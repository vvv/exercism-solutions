pub fn reply(phrase: &str) -> &str {
    if phrase.trim().is_empty() {
        return "Fine. Be that way!";
    }
    let s = phrase.trim_matches(|c: char| !c.is_alphabetic());
    if !s.is_empty() && s.to_uppercase() == s {
        "Whoa, chill out!"
    } else if phrase.trim_right().ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
