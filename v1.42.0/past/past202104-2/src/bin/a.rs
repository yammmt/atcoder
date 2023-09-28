use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    for (i, c) in s.iter().enumerate() {
        if (i == 3 && *c != '-') || (i != 3 && *c == '-') {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
