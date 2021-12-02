// 31min 数問

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
            com.inv[i] = com.mod_base
                - com.inv[com.mod_base as usize % i] * (com.mod_base / i as i64) % com.mod_base;
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
    }
    let d = 1_000_000_007i64;

    // 長さ N の 1 からなる数列を用意し, M の素因数を代入していく組み合わせ数
    // M が同じ素因数を複数もたなければ組み合わせの式そのままだが重複を消さねばならない
    // 2^30 > 10^9 より, 最大ケースは配列長 10^5 に 30 個の約数を分配する場合より小さい
    let com = Com::new(1_000_000, d);

    let mut prime_num = HashMap::new();
    let mut cur_m = m;
    let mut divisor = 2;
    while cur_m > 1 {
        if cur_m % divisor == 0 {
            let cnt = prime_num.entry(divisor).or_insert(0);
            *cnt += 1;
            cur_m /= divisor;
        } else if divisor * divisor > cur_m {
            let cnt = prime_num.entry(cur_m).or_insert(0);
            *cnt += 1;
            break;
        } else {
            divisor += 1;
        }
    }

    let mut ans = 1;
    for v in prime_num.values() {
        ans = (ans * com.com(n + v - 1, *v)) % d;
    }

    println!("{}", ans);
}
