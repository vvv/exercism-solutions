pub fn sum_of_multiples(n: u32, ds: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for x in 1..n {
        if ds.iter().any(|&d| x % d == 0) {
            sum += x;
        }
    }
    sum
}
