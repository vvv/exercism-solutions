pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    let word = sorted_chars(word);
    let mut res = vec![];
    for &w in inputs.iter() {
        let s: String = sorted_chars(w).into_iter().collect();
        if sorted_chars(w) == word {
            res.push(w);
        }
    }
    res
}

fn sorted_chars(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars
}
