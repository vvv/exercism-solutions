pub struct Sieve;

impl Sieve {
    pub fn primes_up_to(n: i32) -> Vec<i32> {
        (2..n+1).fold(vec![], |mut primes, x| {
            if primes.iter().all(|&p| x % p != 0) {
                primes.push(x);
            };
            primes
        })
    }
}
