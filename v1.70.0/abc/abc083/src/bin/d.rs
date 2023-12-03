// ARC

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = s.len();
    for i in 1..s.len() {
        if s[i] != s[i - 1] {
            ans = ans.min(i.max(s.len() - i));
        }
    }

    println!("{ans}");
}
