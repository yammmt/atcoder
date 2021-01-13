// -*- coding:utf-8-unix -*-

// 48.5min
// いもす配列の操作量で大分詰まった

// 問題文が読めない
// [1, 3] が与えられた場合の移動量は 1 or 2 or 3 と読んで良いらしい

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        lrk: [(usize, usize); k],
    }
    let d = 998244353;

    let mut imos = vec![0i64; n + 1];
    imos[1] = 1;
    imos[2] = -1;
    let mut ans = 0;
    for i in 1..n + 1 {
        // そのマスに辿り着く方法の数
        ans = (ans + d + imos[i]) % d;
        if ans == 0 {
            continue;
        }

        for lr in &lrk {
            if i + lr.0 > n {
                continue;
            }

            imos[i + lr.0] = (imos[i + lr.0] + ans) % d;
            if i + lr.1 + 1 <= n {
                imos[i + lr.1 + 1] = (imos[i + lr.1 + 1] + d - ans) % d;
            }
        }
    }

    println!("{}", ans);
}
