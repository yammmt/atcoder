use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [u128; n],
    }

    let a_max = an.iter().max().unwrap();
    for a in &an {
        println!("{}", (1_000_000_000 * *a + a_max / 2) / a_max);
    }
}
