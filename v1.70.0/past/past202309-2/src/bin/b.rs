use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    println!("{}", if a == b && b == c { "Yes" } else { "No" });
}
