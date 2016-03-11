pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    let word = word.to_lowercase();
    let word_sorted = sorted(&word);
    inputs.to_vec().into_iter().filter(|&w| {
        let wl = w.to_lowercase();
        wl != word && sorted(&wl) == word_sorted
    }).collect()
}

fn sorted(word: &String) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
