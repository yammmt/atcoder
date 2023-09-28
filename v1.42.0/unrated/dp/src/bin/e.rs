// :fu: 21-03 どうして

use proconio::input;

const NOT_YET: usize = std::usize::MAX / 2;

fn main() {
    input! {
        n: usize,
        w: usize,
        wvn: [(usize, usize); n],
    }

    // その価値を達成する最小の重さ
    let mut dp = vec![NOT_YET; 100_002];
    dp[0] = 0;
    for wv in &wvn {
        let mut new_dp = dp.clone();
        for i in 0..dp.len() {
            let next_i = i + wv.1;
            if dp[i] == NOT_YET || next_i >= dp.len() {
                continue;
            }

            new_dp[next_i] = dp[next_i].min(dp[i] + wv.0);
        }
        dp = new_dp;
    }
    // println!("{:?}", dp);

    for i in (0..dp.len()).rev() {
        if dp[i] <= w {
            println!("{}", i);
            return;
        }
    }
}
