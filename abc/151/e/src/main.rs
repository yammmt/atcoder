// 二項係数ライブラリないと無理

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

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut an: [i64; n],
    }
    let d = 1_000_000_007;
    an.sort_unstable();

    let com = Com::new(n, d);

    let mut sum_min = 0;
    for i in 0..n - k + 1 {
        sum_min = (sum_min + ((an[i] * com.com(n - i - 1, k - 1)) % d)) % d;
    }

    let mut sum_max = 0;
    for i in k - 1..n {
        sum_max = (sum_max + ((an[i] * com.com(i, k - 1)) % d)) % d;
    }

    println!(
        "{}",
        (sum_max + d - sum_min) % d
    );
}
