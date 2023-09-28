use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    println!(
        "{}",
        if s.last().unwrap() == &'r' {
            "er"
        } else {
            "ist"
        }
    );
}
