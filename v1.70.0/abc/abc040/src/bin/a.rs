use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
    }
    println!("{}", (x - 1).min(n - x));
}
