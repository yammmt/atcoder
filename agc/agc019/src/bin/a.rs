// 5min

use proconio::input;

fn main() {
    input! {
        q: i64,
        h: i64,
        s: i64,
        d: i64,
        n: i64,
    }

    let mut ans = 0;

    let cheapest_2l = (q * 8).min((h * 4).min((s * 2).min(d)));
    if n >= 2 {
        ans += n / 2 * cheapest_2l;
    }
    if n % 2 == 1 {
        let cheapest_1l = (q * 4).min((h * 2).min(s));
        ans += cheapest_1l;
    }

    println!("{}", ans);
}
