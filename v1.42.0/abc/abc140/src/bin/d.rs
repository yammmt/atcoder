// 50min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut chg = 0;
    for i in 1..n {
        if s[i] != s[i - 1] {
            chg += 1;
        }
    }

    println!(
        "{}",
        (n as isize - 1) - 0.max(chg - 2 * k as isize)
    );
}
