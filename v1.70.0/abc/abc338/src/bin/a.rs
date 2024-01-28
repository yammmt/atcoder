use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    for i in 0..s.len() {
        if (i == 0 && s[i].is_lowercase()) || (i > 0 && s[i].is_uppercase()) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
