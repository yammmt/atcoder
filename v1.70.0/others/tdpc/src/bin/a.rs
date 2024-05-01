use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        pn: [usize; n],
    }

    let pmax = pn.iter().sum::<usize>();
    let mut dp = vec![false; pmax + 1];
    dp[0] = true;
    for p in pn {
        let mut cur = vec![false; pmax + 1];
        for i in 0..pmax + 1 {
            if dp[i] {
                cur[i] = true;
                cur[i + p] = true;
            }
        }
        dp = cur;
    }

    let ans = dp.iter().filter(|d| **d).count();
    println!("{ans}");
}
