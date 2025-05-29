use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut an: [usize; n],
    }
    an.sort_unstable();

    let mut ans = usize::MAX;
    for i in 0..=k {
        ans = ans.min(an[i + n - k - 1] - an[i]);
    }

    println!("{ans}");
}
