// 同方針:
// https://atcoder.jp/contests/abc252/editorial/4012

use proconio::fastout;
use proconio::input;

const A_MAX: usize = 200000;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // 全数出現頻度数えて mC2 * 残り個数で 2 要素が一致する場合を引いて,
    // mC3 で 3 要素一致する場合を引く

    let mut cnt = vec![0; A_MAX + 1];
    for a in &an {
        cnt[*a] += 1;
    }

    let mut ans = n * (n - 1) * (n - 2) / 3 / 2;
    for c in &cnt {
        if *c < 2 {
            continue;
        }

        // 2 要素一致
        ans -= (c * (c - 1) / 2) * (n - c);
        // 3 要素一致
        ans -= c * (c - 1) * (c - 2) / 3 / 2;
    }

    println!("{ans}");
}
