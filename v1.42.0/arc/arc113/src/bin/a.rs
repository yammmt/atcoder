// :fu: 数問 21-02

use proconio::input;

fn main() {
    input! {
        k: u64,
    }

    let mut ans = 0;
    for a in 1..k + 1 {
        for b in 1..k + 1 {
            if a * b > k {
                break;
            }

            ans += k / (a * b);
        }
    }

    println!("{}", ans);
}
