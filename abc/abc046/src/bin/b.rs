// 1.5min

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = k;
    for _ in 0..n - 1 {
        ans *= k - 1;
    }
    println!("{}", ans);
}
