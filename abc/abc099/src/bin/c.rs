// 12min 2WA
// WA: dp[0] 起点の計算

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut vsix = vec![];
    let mut vnine = vec![];

    let mut six = 6;
    while six <= n {
        vsix.push(six);
        six *= 6;
    }

    let mut nine = 9;
    while nine <= n {
        vnine.push(nine);
        nine *= 9;
    }

    let mut dp = vec![n; n + 1];
    dp[0] = 0;
    for i in 0..n {
        dp[i + 1] = dp[i + 1].min(dp[i] + 1);

        for a in &vsix {
            if i + *a > n {
                break;
            }

            dp[i + *a] = dp[i + *a].min(dp[i] + 1);
        }
        for a in &vnine {
            if i + *a > n {
                break;
            }

            dp[i + *a] = dp[i + *a].min(dp[i] + 1);
        }
    }

    println!("{}", dp[n]);
}
