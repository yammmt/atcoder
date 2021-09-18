use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    println!(
        "{}",
        if s[n - 1] == 'o' {
            "Yes"
        } else {
            "No"
        }
    );
}
