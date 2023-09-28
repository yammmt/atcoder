// :fu: 21-03

use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    }

    println!(
        "{}",
        if (b - a) % 2 == 0 {
            (b - a) / 2
        } else {
            (a - 1).min(n - b) + 1 + (b - a - 1) / 2
        }
    );
}
