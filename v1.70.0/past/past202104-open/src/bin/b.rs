use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut i = 1;
    while i < s.len() {
        if s[i] == 'o' {
            println!("{}", (i - 1) / 4 + 1);
            return;
        }
        i += 4;
    }

    println!("none");
}
