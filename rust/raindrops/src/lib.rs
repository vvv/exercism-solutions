pub fn raindrops(x: i32) -> String {
    let mut s = String::new();
    if x % 3 == 0 { s.push_str("Pling"); }
    if x % 5 == 0 { s.push_str("Plang"); }
    if x % 7 == 0 { s.push_str("Plong"); }
    if s.is_empty() {
        x.to_string()
    } else {
        s
    }
}
