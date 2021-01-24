use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: Chars,
    }

    println!(
        "{}",
        if c[0] == c[1] && c[1] == c[2] {
            "Won"
        } else {
            "Lost"
        }
    );
}
