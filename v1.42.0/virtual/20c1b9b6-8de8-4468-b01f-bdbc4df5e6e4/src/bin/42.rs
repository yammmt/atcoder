use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
    }

    let mut primes = HashSet::new();
    for &a in &an {
        // 素因数分解
        let mut aa = a;
        let mut divider = 2;
        while aa > 1 && divider * divider <= a {
            if aa % divider == 0 {
                aa /= divider;
                primes.insert(divider);
            } else {
                divider += 1;
            }
        }
        if aa > 1 {
            primes.insert(aa);
        }
    }
    // println!("{:?}", primes);

    let mut is_ans = vec![true; m + 1];
    for &p in &primes {
        let mut pp = p as usize;
        while pp <= m {
            is_ans[pp] = false;
            pp += p;
        }
    }

    let mut vans = vec![1];
    for i in 2..=m {
        if is_ans[i] {
            vans.push(i);
        }
    }

    println!("{}", vans.len());
    for a in &vans {
        println!("{}", a);
    }
}
