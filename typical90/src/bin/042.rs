// :fu: 21-05 普通の DP なんだろうが好きじゃない

use proconio::input;

fn main() {
    input! {
        k: usize,
    }
    let d = 1_000_000_007;

    // dp[i][j]: 和が i で mod 9 が j
    let mut dp = vec![vec![0; 9]; k + 1];
    dp[0][0] = 1;
    for a in 0..k {
        for b in 0..9 {
            // println!("{} {}", a, b);
            for c in 1..10 {
                let next_i = a + c;
                let next_j = (10 * b + c) % 9;
                if next_i > k {
                    break;
                }

                // println!("  {} {}", next_i, next_j);
                dp[next_i][next_j] = (dp[next_i][next_j] + dp[a][b]) % d;
            }
        }
    }

    println!("{}", dp[k][0]);
}
