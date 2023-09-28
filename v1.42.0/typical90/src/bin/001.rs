// :fu: :fu: :fu: 21-04 実装が何故かとても苦手

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        k: usize,
        an: [i64; n],
    }

    let mut pass = 0;
    let mut fail = l + 1;

    while fail - pass > 1 {
        // println!("{} {}", pass, fail);
        let mid = (pass + fail) / 2;
        let mut cur_k = 0;
        let mut cur_l = 0;
        for a in &an {
            // 残りの部分も見なければ分割可能か判断がつかない
            if *a - cur_l >= mid && l - *a >= mid {
                cur_k += 1;
                cur_l = *a;
            }
        }
        if cur_k >= k {
            // 何回か分割をやめてもスコアは変わらない
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
