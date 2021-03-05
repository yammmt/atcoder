// 3.5min

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    let mut ans = b;
    let gedoku = a + b;
    ans += (gedoku + 1).min(c);

    println!("{}", ans);
}
