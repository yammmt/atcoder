use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: u32,
        a: u32,
    }
    println!("{}", (h + a - 1) / a);
}
