struct Cell {
    value: usize,
    reject: bool
}

pub fn primes_up_to(limit: usize) -> Vec<usize> {
    if limit < 2 {
        vec![]
    } else {
        let nr_elems = limit-1;
        let mut xs = Vec::with_capacity(nr_elems);
        for x in 2..limit+1 {
            xs.push(Cell { value: x, reject: false });
        }
        let mut primes = vec![];
        let mut start = 0;
        while start < nr_elems {
            let prime = xs[start].value;
            primes.push(prime);
            // Mark multiples of `prime'.
            let mut cur = start + prime;
            while cur < nr_elems {
                xs[cur].reject = true;
                cur += prime;
            }
            // Find position of next prime.
            start += 1;
            while start < nr_elems && xs[start].reject {
                start += 1;
            }
        }
        primes
    }
}
