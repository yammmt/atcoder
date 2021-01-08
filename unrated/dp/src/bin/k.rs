// :fu:

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    // 負けの盤面を相手に押し付けられれば勝ち
    let mut dp = vec![false; k + 1];
    for i in 0..k {
        if dp[i] {
            continue;
        }

        for a in &an {
            if i + *a > k {
                break;
            }

            dp[i + *a] = true;
        }
    }
    // println!("{:?}", dp);

    println!(
        "{}",
        if dp[k] {
            "First"
        } else {
            "Second"
        }
    );
}
