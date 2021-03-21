use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    // 二文字連続
    for (i, c) in s.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if *c == s[i - 1] {
            println!("{} {}", i, i + 1);
            return;
        }
    }

    // 三文字中二文字
    for (i, c) in s.iter().enumerate() {
        if i < 2 {
            continue;
        }

        if *c == s[i - 2] {
            println!("{} {}", i - 1, i + 1);
            return;
        }
    }

    println!("-1 -1");
}
