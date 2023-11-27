use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lrq: [(usize, usize); q],
    }

    let mut cusum = vec![0; n + 1];
    for i in 2..=n {
        cusum[i] = cusum[i - 1];
        if s[i - 1] == s[i - 2] {
            cusum[i] += 1;
        }
    }

    for lr in lrq {
        println!("{}", cusum[lr.1] - cusum[lr.0]);
    }
}
