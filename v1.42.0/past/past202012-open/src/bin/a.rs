use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    for i in 2..5 {
        if s[i - 2] == s[i] && s[i - 1] == s[i] {
            println!("{}", if s[i] == 'o' { 'o' } else { 'x' });
            return;
        }
    }

    println!("draw");
}
