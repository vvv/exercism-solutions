pub fn verse(n: u8) -> String {
    match n {
        0 => format!(
            "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!(
            "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!(
            "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n"),
        _ => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.
Take one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1)
    }
}

pub fn sing(from: u8, to: u8) -> String {
    (to..from+1).rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
