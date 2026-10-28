use rug::Integer;
use rayon::prelude::*;

fn get_primes(lo: u64, hi:u64, base: &[u64], mut visit: impl FnMut(u64)) {
    let block_size: u64 = 1 << 24;
    let mut is_prime = vec![true; block_size as usize];

    let mut block_start = lo.max(2);
    while block_start < hi {
        let block_end = (block_start + block_size).min(hi);
        is_prime.fill(true);

        for &p in base {
            if p*p >= block_end {
                break;
            }
            let first_mul = (p * p).max(block_start.div_ceil(p) * p);
//          Chop multiples of p
            for mul in (first_mul..block_end).step_by(p as usize) {
                is_prime[(mul - block_start) as usize] = false;
            }
        }
        for n in block_start..block_end {
            if is_prime[(n - block_start) as usize] {
                visit(n);
            }
        }
        block_start = block_end;
    }
}

fn small_primes(n: u64) -> Vec<u64> {
//   Handle small primes separately
    let mut is_prime = vec![true; n as usize + 1];
    let mut primes = Vec::new();
    for i in 2..=n {
        if is_prime[i as usize] {
            primes.push(i);
            for mul in (i * i..=n).step_by(i as usize) {
                is_prime[mul as usize] = false;
            }
        }
    }
    primes
}


fn main() {
    const STEPS: usize = 1_000_000;
    const NUM_CORES: u64 = 4;
    const B: u64 = 1_000_000;
    const W: u64 = 1 << 14;
    const NUM_CHUNKS: u64 = B.div_ceil(W);
    let base = small_primes((B as f64).sqrt() as u64 + 1);
//     let mut div = vec![false; STEPS];
    let mut div = (0..NUM_CHUNKS).into_par_iter()
        .fold(|| vec![false; STEPS], |mut div, chunk| {
            let lo = chunk * W;
            let hi = (lo + W).min(B);
            //  Build div where div[i] = false if index i passes sieve
            get_primes(lo, hi, &base, |p| {
                let (mut a, mut b, mut c) = (1u64, 1, 1);
                for step in 0..STEPS {
                    let mut r = a + b + c ;
            // Instead of modulo
                    r = r.min(r.wrapping_sub(p));
                    r = r.min(r.wrapping_sub(p));
                    a = b;
                    b = c;
                    c = r;
                    if r == 0 {
                        div[step] = true;
                    }
                }
            });
            div
        })
        .reduce(|| vec![false; STEPS], |mut a, b| {
            for i in 0..STEPS{ a[i] |= b[i]; }
            a
        });

    println!("Made Div");
    let mut cur = Integer::from(1);
    let mut mid = Integer::from(1);
    let mut last = Integer::from(1);
    let _i: u128 = 0;
    println!("{}", last);
    println!("{}", mid);
    println!("{}", cur);
    let mut count = 0;
//     1 to 100k run normally
    for i in 0..STEPS {
        let cur_copy = cur.clone();
        let mut next = Integer::from(&cur + &mid);
        next += last;
        cur = next;
        last = mid;
        mid = cur_copy;
        if !div[i] {
//             println!("{}", cur);
            count += 1
        }

//         if !div[i] && cur.is_probably_prime(30) != IsPrime::No {
//          if cur.is_probably_prime(30) != IsPrime::No {
//             println!("Index {i} has probable prime {cur}");
//         }
    }
    println!("{}", count);

}