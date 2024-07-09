use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mn: [usize; n],
    }

    println!("{}", mn.iter().map(|&m| 80 - m.min(80)).sum::<usize>());
}
