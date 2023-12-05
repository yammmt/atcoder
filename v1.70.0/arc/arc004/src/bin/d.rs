// 並の高速化で WA が取れないが見当つかず時間切れ

use proconio::fastout;
use proconio::input;

const MOD: i64 = 1_000_000_007;

#[derive(Debug)]
struct Com {
    fac: Vec<i64>,
    finv: Vec<i64>,
    inv: Vec<i64>,
    mod_base: i64,
}

impl Com {
    fn new(n: usize, m: i64) -> Self {
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

#[fastout]
fn main() {
    input! {
        mut n: i64,
        m: i64,
    }

    if m.abs() == 1 {
        println!("1");
        return;
    }

    let n_original = n;
    let n_negative = n < 0;
    n = n.abs();
    let com = Com::new((2 * m + 1) as usize, MOD);
    let mut ans_posi = 1i64;
    let mut ans = 0;

    let mut divisor = 2;
    while n > 1 && divisor * divisor <= n_original.abs() {
        let mut cnt = 0;
        while n % divisor == 0 {
            cnt += 1;
            n /= divisor;
        }
        if cnt > 0 {
            let muled = com.com((cnt + m - 1) as usize, (m - 1) as usize);
            ans_posi = (ans_posi * muled) % MOD;
        }
        divisor += 1;
    }
    if n != 1 {
        ans_posi = (ans_posi * m) % MOD;
    }
    // println!("{ans_posi}");

    // 負数の分配を最後に入れる
    for nega_num in 0..=m {
        if (nega_num % 2 == 0 && n_negative) || (nega_num % 2 == 1 && !n_negative) {
            continue;
        }

        // nega_num 個の負符号を m 箇所に割り振る
        // 明らかに m >= nega_num であり m 個のうち nega_num 個を選ぶ組み合わせ
        let muled = com.com(m as usize, nega_num as usize);
        ans = (ans + (ans_posi * muled)) % MOD;
    }

    println!("{ans}");
}
