extern crate chrono;
use chrono::*;

// chrono documentation:
//   1. https://lifthrasiir.github.io/rust-chrono/chrono/index.html
//   2. https://docs.rs/chrono/0.2.25/chrono/

pub fn after(dt: DateTime<UTC>) -> DateTime<UTC> {
    dt + Duration::seconds(1_000_000_000)
}
