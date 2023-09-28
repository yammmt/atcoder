// :fu:

use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let mut ans = 0;
    for b in k + 1..n + 1 {
        let cycle = n / b;
        ans += cycle * (b - k).max(0);
        ans += (n % b - (k - 1).max(0)).max(0);
    }
    println!("{}", ans);
}
