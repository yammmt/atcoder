// :fu: :fu:

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
    }
    an.sort_unstable();

    let mut asum = an.iter().sum::<i64>();
    let mut ans = 0;
    for i in 0..n - 1 {
        asum -= an[i];
        ans += asum - an[i] * (n as i64 - i as i64 - 1);
    }
    println!("{}", ans);
}
