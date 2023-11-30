use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        n: usize,
        an: [usize; n],
    }
    let asum = an.iter().sum::<usize>();
    println!("{}", if asum >= h { "Yes" } else { "No" });
}
