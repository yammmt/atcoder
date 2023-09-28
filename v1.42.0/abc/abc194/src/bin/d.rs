// コンプガチャの期待値は天才発想では…？

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0.0;
    for i in 1..n {
        ans += n as f64 / (n as f64 - i as f64);
    }

    println!("{}", ans);
}
