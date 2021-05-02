// NOT WORK "すべての" を誤読

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
        m: usize,
        xyzm: [(usize, usize, usize); m],
    }
    // 多分超えない...
    let com = Com::new(n, 1_000_000_007);

    // let mut ans = 0i64;
    for xyz in &xyzm {
        let mut lkaijo = 1;
        let mut gkaijo = 1;
        for i in 1..xyz.0 + 1 {
            lkaijo *= i;
        }
        for i in 1..(n - xyz.0 + 1) {
            gkaijo *= i;
        }
        println!("{} {}", lkaijo, gkaijo);

        for i in 1..xyz.2 + 1 {
            println!("i: {}", i);
            // Y_i イカの数が i 個
            let lesser = com.com(xyz.1, i) * com.com(n - xyz.1, n - i - 2);
            println!("  {}C{} * {}C{}", xyz.1, i, n - xyz.1, n - i - 2);
            println!("  lesser: {}", lesser);

            // ans += lesser * lkaijo * gkaijo;
        }
    }

    // println!("{}", ans);
}
