// 10min 1WA

use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
    }

    let ans = if a > b || (n == 1 && a != b) {
        0
    } else if n <= 2 {
        1
    } else {
        (n - 2) * b - (n - 2) * a + 1
    };
    println!("{}", ans);
}
