/// See http://exercism.io/submissions/3dc698f957dc46b7afcb3f7a66c39a51

// pub fn nth(n: usize) -> Result<i32, &'static str> {
//     if n == 0 {
//         return Err("invalid n");
//     }

//     // Fill in primes < 6 so we can skip 2/3 of candidates.
//     let mut primes: Vec<i32> = vec![2, 3, 5];
//     let mut candidate: i32 = 5;
//     while primes.len() < n {
//         while {
//             candidate += if candidate % 6 == 5 { 2 } else { 4 };
//             (&primes)
//                 .into_iter()
//                 .take_while(|&p| p * p <= candidate)
//                 .any(|&p| candidate % p == 0)
//         } {}
//         primes.push(candidate);
//     }
//     Ok(primes[n - 1])
// }

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
