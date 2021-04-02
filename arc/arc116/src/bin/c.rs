// :fu: :fu: 21-04 標準的な水色という雰囲気
// 公式解説 (HTML) の行間が読めないが YouTube に救われる

use proconio::input;
use std::collections::HashMap;

#[derive(Debug)]
struct Com {
    fac: Vec<i64>,
    finv: Vec<i64>,
    inv: Vec<i64>,
    n_max: usize,
    mod_base: i64,
}

impl Com {
    fn new(n: usize, m: i64) -> Self {
        let mut com = Com {
            fac: vec![0; n + 1],
            inv: vec![0; n + 1],
            finv: vec![0; n + 1],
            n_max: n,
            mod_base: m,
        };

        com.fac[0] = 1;
        com.fac[1] = 1;
        com.inv[1] = 1;
        com.finv[0] = 1;
        com.finv[1] = 1;

        for i in 2..n + 1 {
            com.fac[i] = com.fac[i - 1] * i as i64 % com.mod_base;
            com.inv[i] = com.mod_base - com.inv[com.mod_base as usize % i] * (com.mod_base / i as i64) % com.mod_base;
            com.finv[i] = com.finv[i - 1] * com.inv[i] % com.mod_base;
        }

        com
    }

    // nCk mod m
    fn com(&self, n: usize, k: usize) -> i64 {
        if n < k {
            return 0;
        }

        self.fac[n] * (self.finv[k] * self.finv[n - k] % self.mod_base) % self.mod_base
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let d = 998_244_353;

    let com = Com::new(n + m, d);

    let mut minor_prime = vec![std::usize::MAX / 2; m + 1];
    minor_prime[0] = 0;
    minor_prime[1] = 0;
    for p in 2..m + 1 {
        if minor_prime[p] != std::usize::MAX / 2 {
            continue;
        }

        minor_prime[p] = 0;
        let mut cur = p;
        while cur <= m {
            minor_prime[cur] = minor_prime[cur].min(p);
            cur += p;
        }
    }
    // println!("{:?}", minor_prime);

    // all 1
    let mut ans = 1;
    for an in 2..m + 1 {
        // println!("an: {}", an);
        let mut primes = HashMap::new();
        let mut cur = an;
        loop {
            let devided_by = minor_prime[cur];
            if devided_by == 0 {
                let cnt = primes.entry(cur).or_insert(0);
                *cnt += 1;
                break;
            } else {
                let cnt = primes.entry(devided_by).or_insert(0);
                *cnt += 1;
                cur /= minor_prime[cur];
            }
        }
        // println!("  {:?}", primes);
        let mut cur = 1;
        for v in primes.values() {
            cur = (cur * com.com(v + n - 1, n - 1)) % d;
        }
        // println!("  {}", cur);
        ans = (ans + cur) % d;
    }

    println!("{}", ans);
}
