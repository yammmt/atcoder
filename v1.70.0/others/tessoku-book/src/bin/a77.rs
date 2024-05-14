// シンプルに苦手

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        an: [usize; n],
    }

    // スコア: 最低 0 点は達成可能
    let mut pass = 0;
    let mut fail = l + 1;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        // println!("{fail} {pass}, mid: {mid}");
        let mut k_cur = 0;
        let mut l_prev = 0;
        let mut l_cur = 0;
        for &a in &an {
            // 左: スコア mid を達成するには, 超えた直後で切る
            // 右: 切った後の残り, 右側の長さも mid 以上ないと制約未達となり切れない
            //     あるいは, ループ抜けた後に L-an.last 次第で必要な切断回数を +1 する
            if l_cur + a - l_prev >= mid && l - a >= mid {
                // println!("  cut: after {a}");
                k_cur += 1;
                l_cur = 0;
            } else {
                l_cur += a - l_prev;
            }
            l_prev = a;
        }

        // println!("  k_cur: {k_cur}");
        // 切れ目が k 未満なら k 個まで切れ目を増やすとスコアは悪化するので fail
        if k_cur >= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{pass}");
}
