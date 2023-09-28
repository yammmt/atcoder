use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    for (i, c) in s.iter().enumerate() {
        if *c == '1' {
            println!("{}", if i % 2 == 0 { "Takahashi" } else { "Aoki" });
            return;
        }
    }
}
