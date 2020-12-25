// 7min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    for i in 0..n {
        // s[i] 以降が t[j] と共通
        for j in 0..n - i {
            if s[i + j] != t[j] {
                break;
            }

            if j == n - i - 1 {
                println!("{}", n + i);
                return;
            }
        }
    }

    println!("{}", 2 * n);
}
