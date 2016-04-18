pub fn hamming_distance(a: &str, b: &str) -> Result<u32, &'static str> {
    if a.len() == b.len() {
        let mut n = 0;
        for (x, y) in a.chars().zip(b.chars()) {
            if x != y {
                n += 1;
            }
        }
        Ok(n)
    } else {
        Err("inputs of different length")
    }
}
