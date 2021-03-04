// :fu: 21-03 Giatama というより知っていれば系
// https://31536000.hatenablog.com/entry/2018/05/21/014635

use proconio::input;

fn main() {
    input! {
        n: usize,
        pn: [usize; n],
    }

    let mut dp = vec![0; n + 1];
    for p in &pn {
        dp[*p] = dp[*p].max(dp[*p - 1] + 1);
    }

    println!("{}", n - dp.iter().max().unwrap());
}
