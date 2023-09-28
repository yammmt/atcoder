// :fu: :fu: :fu: :fu: 21-11 数問 (組み合わせがわかっていない)
// https://blog.hamayanhamayan.com/entry/2020/02/24/092051

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
        k: usize,
    }
    let d = 1_000_000_007;
    let k = k.min(n - 1);

    let mut ans = 0;
    let com = Com::new(n + k + 1, d);
    // 誰もいない部屋の個数を i に固定する
    for i in 0..n.min(k + 1) {
        // i 部屋が 0 人であるので残りの部屋数は n - i 個
        // n - i 部屋に 1 人ずつは配置しなくてはならないので, 自由に配置できるのは n - (n - i) 人
        // n - (n - i) = i 人と部屋数に相当する n - i - 1 個の仕切りを並べるので
        // (i + n - i - 1)Ci = (n - 1)Ci あるいは (n - 1)C(n - i - 1)
        ans = (ans + com.com(n, i) * com.com(n - 1, i)) % d;
    }

    println!("{}", ans);
}
