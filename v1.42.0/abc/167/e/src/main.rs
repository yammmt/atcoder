// :fu: 21-03 ncr_mod も高速化しないと TLE する

use proconio::input;

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
        m: i64,
        k: i64,
    }
    let d = 998244353;

    let mut mm1_pow = vec![0; n + 1];
    mm1_pow[0] = 1;
    mm1_pow[1] = m - 1;
    for i in 2..n + 1 {
        mm1_pow[i as usize] = (mm1_pow[i as usize - 1] * (m - 1)) % d;
    }
    let com = Com::new(n, d);

    let mut ans = 0;
    for i in 0..k + 1 {
        let cur = ((m * com.com(n - 1, i as usize) % d * mm1_pow[n - 1 - i as usize])) % d;
        ans = (ans + cur) % d;
    }

    println!("{}", ans);
}
