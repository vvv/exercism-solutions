use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut res = HashMap::new();
    for w in s.replace(|c: char| !c.is_alphanumeric(), " ").split_whitespace() {
        *res.entry(w.to_lowercase().to_string()).or_insert(0) += 1;
    }
    res
}
