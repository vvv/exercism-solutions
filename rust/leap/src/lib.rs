pub fn is_leap_year(year: u32) -> bool {
    let div_by = |n| year % n == 0;
    div_by(4) && !div_by(100) || div_by(400)
}
