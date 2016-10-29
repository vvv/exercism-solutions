pub struct Sieve;

impl Sieve {
    pub fn primes_up_to(n: i32) -> Vec<i32> {
        (2..n+1).fold(vec![], |mut acc, i| {
            match acc.iter().all(|&x| i % x != 0) {
                true => {acc.push(i); acc},
                false => acc
            }
        })
    }
}
