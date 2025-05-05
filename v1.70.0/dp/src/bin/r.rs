// 行列累乗

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    const MOD: usize = 1_000_000_007;
    input! {
        n: usize,
        mut k: usize,
        ann: [[usize; n]; n],
    }

    let matrix_mul = |a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>| {
        let nn = a.len();
        let mut ret = vec![vec![0; nn]; nn];
        for i in 0..nn {
            for j in 0..nn {
                for k in 0..nn {
                    ret[i][j] += a[i][k] * b[k][j];
                    ret[i][j] %= MOD;
                }
            }
        }
        ret
    };

    // 初期値は単位ベクトル: 各頂点に至るパスが一つしかないため
    let mut r = vec![vec![0; n]; n];
    for i in 0..n {
        r[i][i] = 1;
    }

    // 個人的に慣れ親しんでいない繰り返し二乗法の形
    // k を二進法で表して, 最下位ビットから加算していく形
    let mut t = ann;
    while k > 0 {
        if k % 2 == 1 {
            r = matrix_mul(&r, &t);
        }
        t = matrix_mul(&t, &t);

        k /= 2;
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans += r[i][j];
            ans %= MOD;
        }
    }

    println!("{ans}");
}
