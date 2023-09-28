use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lrq: [(usize, usize); q],
    }

    let mut cusum = vec![0; n];
    for i in 1..n {
        cusum[i] = if s[i] == 'C' && s[i - 1] == 'A' {
            cusum[i - 1] + 1
        } else {
            cusum[i - 1]
        };
    }

    for lr in &lrq {
        println!("{}", cusum[lr.1 - 1] - cusum[lr.0 - 1]);
    }
}
