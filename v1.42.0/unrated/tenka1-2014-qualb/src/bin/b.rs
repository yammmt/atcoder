// 11.5min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        tn: [Chars; n],
    }
    let d = 1_000_000_007;

    let mut dp = vec![0; s.len() + 1];
    dp[0] = 1;
    for i in 0..s.len() {
        for t in &tn {
            // println!("{:?}", t);
            if i + t.len() > s.len() || dp[i] == 0 {
                continue;
            }

            let mut pass = true;
            for (j, c) in t.iter().enumerate() {
                if *c != s[i + j] {
                    pass = false;
                    break;
                }
            }
            if pass {
                dp[i + t.len()] = (dp[i + t.len()] + dp[i]) % d;
            }
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp [s.len()]);
}
