use ac_library::modint::ModInt998244353 as Mint;
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

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut an: [usize; n],
    }
    an.sort_unstable();

    // ai は相異なるので考察がちょっと楽できそう
    // 割り算しなくとも最大値全部足して最小値全部引けば求まりそう
    // 全体の最大値が区間最大値となる場合は自身が選ばれる + 自身より大きい要素が選ばれない
    // 二項係数を前計算して使い回すやつっぽい

    // => sounansya さん解説と同じっぽい

    let com = Com::new(n, MOD);

    let mut ans = Mint::new(0);
    for i in 0..n - k + 1 {
        // an[n-i-1] が最大値となる場合の数は, 自身が選ばれかつ残りがすべて自身より小さい場合の数
        // 自身より小さい数が n-i-1 個
        ans += an[n - i - 1] * com.com(n - i - 1, k - 1);
    }

    for i in 0..n - k + 1 {
        // 自身より大きい数は n-i-1 個
        ans -= an[i] * com.com(n - i - 1, k - 1);
    }

    // 期待値だから割る
    ans /= com.com(n, k);

    println!("{ans}");
}
