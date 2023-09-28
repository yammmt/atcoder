// 日本語

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        xn: [i64; n],
    }

    let mut ans = 0;
    for &x in &xn {
        let ta = 2 * x;
        let tb = 2 * (k - x).abs();
        ans += ta.min(tb);
    }

    println!("{}", ans);
}
