// 16.5min 2WA
// WA: 素因数分解の TLE, 素数掛け合わせる回数誤り

// :fu:

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: u64,
        p: u64,
    }

    if n == 1 {
        println!("{}", p);
        return;
    }

    let mut curp = p;
    let mut cur = 2;
    let mut primes = HashMap::new();
    while curp > 1 && cur * cur <= p {
        if curp % cur == 0 {
            let cnt = primes.entry(cur).or_insert(0);
            *cnt +=1;
            curp /= cur;
        } else {
            cur += 1;
        }
    }

    let mut ans = 1;
    for (k, v) in &primes {
        let a = *v / n;
        for _ in 0..a {
            ans *= k;
        }
    }

    println!("{}", ans);
}
