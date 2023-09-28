// :fu: :fu: 21-06
// それなりには撤退されたらしき参加者数ではある

// https://zenn.dev/hal_mat/articles/0c3857b22b6ff1
// editorial はまたもや DP の遷移式から先の行間が読めない ユーザー解説待ち

// 至るところで剰余を取らないと WA

// 2
// 3 1
// => 6 (= 4 + 2)

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }
    let d = 1_000_000_007;

    let mut ansp = ModInt { n: an[0], d };
    let mut ansn = ModInt { n: 0, d };
    // dp[i][j]: i 個目に符号が +/- になる場合の数
    let mut dp = vec![vec![ModInt { n: 0, d }; 2]; n];
    dp[0][0].n = 1;
    for (i, a) in an.iter().skip(1).enumerate() {
        let tmpp = ansp;
        let tmpn = ansn;
        ansp = tmpp + tmpn + ModInt { n: *a, d } * (dp[i][0] + dp[i][1]);
        ansn = tmpp - (dp[i][0] * ModInt { n: *a, d });
        // println!("{} {}", ansp, ansn);

        dp[i + 1][0] = dp[i][0] + dp[i][1];
        dp[i + 1][1] = dp[i][0]
    }
    // println!("{:?}", dp);
    // println!("{} {}", ansp, ansn);

    println!("{}", ansp + ansn);
}

#[derive(Clone, Copy, Debug)]
struct ModInt {
    n: i64,
    d: i64,
}

impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            n: (self.n + rhs.n) % self.d,
            d: self.d,
        }
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            n: if self.n - rhs.n < 0 {
                self.n - rhs.n + self.d
            } else {
                self.n - rhs.n
            },
            d: self.d,
        }
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            n: (self.n * rhs.n) % self.d,
            d: self.d,
        }
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.n)
    }
}
