pub fn sum_of_multiples(n: u32, ds: &Vec<u32>) -> u32 {
    (1..n).filter(|x| ds.iter().any(|d| x % d == 0)).sum()
}
