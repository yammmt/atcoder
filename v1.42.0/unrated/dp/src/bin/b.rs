use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        hn: [i64; n],
    }

    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..k + 1 {
            // println!("i - j: {}", i as i32 - j as i32);
            if i < j {
                continue;
            }

            dp[i] = dp[i].min(dp[i - j] + (hn[i] - hn[i - j]).abs());
        }
        // println!("{:?}", dp);
    }

    println!("{}", dp[n - 1]);
}
