fn set_primes(a: &mut Vec<bool>) {
    let len = a.len();
    for i in 0..len {
        if a[i] {
            let mut n = i*i;
            while n < len {
                a[n] = false;
                n += i;
            }
        }
    }
}

pub fn nth(n: u32) -> Result<u32, &'static str> {
    let ulimit = 1_000_000;
    if n < 1 || n as usize > ulimit {
        return Err("Out of bounds");
    }
    let mut numbers = vec![true; ulimit];
    numbers[0] = false;
    numbers[1] = false;
    set_primes(&mut numbers);

    let mut k = 1;
    for (i, _) in numbers.iter().enumerate().filter(|e| *e.1) {
        if k == n {
            return Ok(i as u32);
        }
        k += 1;
    }
    Err("XXX FIXME")
}
