use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        _n: usize,
        k: Usize1,
        s: Chars,
    }
    for (i, c) in s.into_iter().enumerate() {
        if i <= k || c == 'x' {
            continue;
        }

        println!("{}", i + 1);
        return;
    }
}
