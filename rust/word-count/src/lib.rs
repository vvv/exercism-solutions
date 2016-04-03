use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    s.replace(|c: char| !c.is_alphanumeric(), " ").split_whitespace()
        .fold(HashMap::new(), |mut res, w| {
            *res.entry(w.to_lowercase().to_string()).or_insert(0) += 1;
            res
        })
}
