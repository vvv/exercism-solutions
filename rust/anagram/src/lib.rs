pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    let word = word.to_lowercase();
    let word_sorted = sorted(&word);
    let mut res = vec![];
    for &w in inputs.iter() {
        let wl = w.to_lowercase();
        if wl != word && sorted(&wl) == word_sorted {
            res.push(w);
        }
    }
    res
}

fn sorted(word: &String) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
