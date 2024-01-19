// https://blog.hamayanhamayan.com/entry/2020/05/10/233015
// > 前のブロックと同じにする回数と前のブロックと異なるようにする回数は同じになる

use proconio::fastout;
use proconio::input;

const MOD: usize = 998_244_353;

struct Com {
    fac: Vec<usize>,
    finv: Vec<usize>,
    inv: Vec<usize>,
    mod_base: usize,
}

impl Com {
    fn new(n: usize, m: usize) -> Self {
        let mut com = Com {
            fac: vec![0; n + 1],
            inv: vec![0; n + 1],
            finv: vec![0; n + 1],
            mod_base: m,
        };

        com.fac[0] = 1;
        com.fac[1] = 1;
        com.inv[1] = 1;
        com.finv[0] = 1;
        com.finv[1] = 1;

        for i in 2..n + 1 {
            com.fac[i] = com.fac[i - 1] * i % com.mod_base;
            com.inv[i] =
                com.mod_base - com.inv[com.mod_base % i] * (com.mod_base / i) % com.mod_base;
            com.finv[i] = com.finv[i - 1] * com.inv[i] % com.mod_base;
        }

        com
    }

    // nCk mod m
    fn com(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }

        self.fac[n] * (self.finv[k] * self.finv[n - k] % self.mod_base) % self.mod_base
    }
}

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: usize, p: usize, m: usize) -> usize {
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * repeat_square(n, p - 1, m)) % m
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let com = Com::new(n, MOD);
    let mut ans = 0;
    for i in 0..=k {
        ans = (ans + (com.com(n - 1, i) * m) % MOD * repeat_square(m - 1, n - 1 - i, MOD)) % MOD;
    }

    println!("{ans}");
}
