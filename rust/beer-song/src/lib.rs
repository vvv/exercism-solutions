pub fn verse(n: u8) -> String {
    format!("{} on the wall, {}.\n{}, {} on the wall.\n",
            capitalize(amount(n)), amount(n),
            if n == 0 {
                "Go to the store and buy some more".to_string()
            } else {
                format!("Take {} down and pass it around",
                        if n == 1 { "it" } else { "one" })
            },
            amount(decr(n)))
}

pub fn sing(from: u8, to: u8) -> String {
    assert!(from > to);
    (to..from+1).rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}

fn amount(n: u8) -> String {
    (match n {
        0 => String::from("no more bottles"),
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", n),
    }) + " of beer"
}

fn decr(n: u8) -> u8 {
    if n == 0 { 99 } else { n-1 }
}

fn capitalize(s: String) -> String {
    let mut s = s.chars();
    match s.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(s).collect(),
    }
}
