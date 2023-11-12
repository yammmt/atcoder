use proconio::fastout;
use proconio::input;

static MOD: usize = 998_244_353;

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
        h: usize,
        w: usize,
        k: usize,
        xy1: (usize, usize),
        xy2: (usize, usize),
    }

    // 同じ行/列でいったりきたりし続けていればよい
    // i 回だけ行/列移動した場合の遷移するパスは数式で表せそうだが計算量間に合うはずなので計算する

    let com = Com::new(k, MOD as i64);

    let mut moved_num_except_x1 = vec![0; k + 1];
    let mut moved_num_x1 = vec![0; k + 1];
    let mut moved_num_except_y1 = vec![0; k + 1];
    let mut moved_num_y1 = vec![0; k + 1];
    moved_num_x1[0] = 1;
    moved_num_y1[0] = 1;
    for i in 1..=k {
        moved_num_except_x1[i] =
            ((moved_num_except_x1[i - 1] * (h - 2)) % MOD + moved_num_x1[i - 1]) % MOD;
        moved_num_except_y1[i] =
            ((moved_num_except_y1[i - 1] * (w - 2)) % MOD + moved_num_y1[i - 1]) % MOD;
        moved_num_x1[i] = (moved_num_except_x1[i - 1] * (h - 1)) % MOD;
        moved_num_y1[i] = (moved_num_except_y1[i - 1] * (w - 1)) % MOD;
    }
    // println!("{:?}", moved_num_x1);
    // println!("{:?}", moved_num_y1);
    // println!("{:?}", moved_num_except_x1);
    // println!("{:?}", moved_num_except_y1);

    let mut ans = 0;
    for i in 0..=k {
        // x を i 回操作すると y は k-i 回操作することになる
        let x_num = if xy1.0 == xy2.0 {
            moved_num_x1[i]
        } else {
            moved_num_except_x1[i]
        };
        let y_num = if xy1.1 == xy2.1 {
            moved_num_y1[k - i]
        } else {
            moved_num_except_y1[k - i]
        };

        let cur = (((x_num * y_num) % MOD) * com.com(k, i) as usize) % MOD;
        ans = (ans + cur) % MOD;
        // println!("x: {i}, y: {}", k-i);
        // println!("  {x_num} x {y_num} x {}", com.com(k, i));
        // println!("    cur: {cur}");
    }

    println!("{ans}");
}
