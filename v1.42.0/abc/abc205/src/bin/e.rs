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
        k: usize,
    }
    let d = 1_000_000_007;

    // これまでに w を何個置いたかで DP と思ったが O(N^2) っぽい
    // 余事象を考えると左から i 番目の時点で初めてアウトになる場合は
    // i 番目時点での左の並べ方 x 右の並べ型 - (i - 1) 番目でアウトとなる並べ方、となり O(NlogN) に落ちそう
    // 本当に？
    let com = Com::new(n + m + 1, d);

    let mut ans = com.com(n + m, n);
    let mut ncr_prev = 0;
    for i in k..n + m {
        // アウトとなるのは最後に W を置いた場合のみ
        // 左側に i + 1 個、右側に n + m - i - 1 個
        // i + 1 個置いた時点でアウトとなる場合には白の数は i + 1 個
        // 左側の並べ方
        // let left = com.com(i + 1, );

        // 右側の並べ方
        // let right = com.com(n + m - i - 1, );

        // 直前までにカウントした分は引く
        // let left_right = (left * right) % d;
        // ans = (ans + d - (left_right + d - ncr_prev) % d) % d;

        // 更新
        // ncr_prev = left_right;
    }

    println!("{}", ans);
}
