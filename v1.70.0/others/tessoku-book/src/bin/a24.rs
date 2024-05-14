use proconio::fastout;
use proconio::input;

const INF: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut l = 0;
        let mut r = n;
        while r - l > 1 {
            let mid = (l + r) / 2;
            if an[i] > dp[mid] {
                l = mid;
            } else {
                r = mid;
            }
        }
        dp[r] = dp[r].min(an[i]);
        // println!("{:?}", dp);
        // return;
    }

    for i in (1..=n).rev() {
        if dp[i] != INF {
            println!("{i}");
            return;
        }
    }

    assert!(false);
}
