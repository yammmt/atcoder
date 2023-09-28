// マス固定であればそのマスへの遷移数で DP
// **盤面ごと** の経路数の総和を求める
// 経路を決め打ちしてそれに必要なマスの組み合わせを加算する
// では最大サイズで TLE した

use proconio::input;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
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

fn extgcd(mut a: i64, m: i64) -> u64 {
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u as u64
}

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        hwck: [(usize, usize, char); k],
    }
    let d = 998244353;

    let three_max = repeat_square(3, (h * w - k) as u64, d);
    let three_inv = extgcd(3, d as i64);
    let mut mass_defined = vec![vec!['F'; w]; h];
    for hwc in &hwck {
        mass_defined[hwc.0 - 1][hwc.1 - 1] = hwc.2;
    }

    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = three_max;
    for i in 0..h {
        for j in 0..w {
            match mass_defined[i][j] {
                'F' => {
                    let mv_way = ((dp[i][j] * three_inv) % d * 2) % d;
                    if j + 1 < w {
                        dp[i][j + 1] = (dp[i][j + 1] + mv_way) % d;
                    }
                    if i + 1 < h {
                        dp[i + 1][j] = (dp[i + 1][j] + mv_way) % d;
                    }
                },
                'R' => {
                    if j + 1 < w {
                        dp[i][j + 1] = (dp[i][j + 1] + dp[i][j]) % d;
                    }
                },
                'D' => {
                    if i + 1 < h {
                        dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % d;
                    }
                },
                'X' => {
                    if j + 1 < w {
                        dp[i][j + 1] = (dp[i][j + 1] + dp[i][j]) % d;
                    }
                    if i + 1 < h {
                        dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % d;
                    }
                },
                _ => unreachable!(),
            }
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[h - 1][w - 1]);
}
