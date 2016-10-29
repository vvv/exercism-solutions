pub struct Sieve;

impl Sieve {
    pub fn primes_up_to(n: usize) -> Vec<usize> {
        let mut rejected = vec![false; n+2]; // Size 2 larger than n
                                             // for simpler indexes.
        let mut primes = Vec::with_capacity(p_n_upper_bound(n));

        for i in 2..n+1 {
            if rejected[i] {
                continue;
            }
            primes.push(i);
            // Eliminate multiples of i.
            let mut j = 2*i;
            while j <= n {
                rejected[j] = true;
                j += i;
            }
        }
        primes
    }
}

fn p_n_upper_bound(x: usize) -> usize {
    let x = x as f64;
    (x / (x.ln() - 4.0).abs()) as usize
}
