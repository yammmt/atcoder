use proconio::fastout;
use proconio::input;

static DUMMY: isize = isize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        xyn: [(isize, isize, isize); n],
    }

    let bit_len = 1 << n;
    let mut dp = vec![vec![DUMMY; n]; bit_len];
    // 都市 1 は訪問済み
    dp[1][0] = 0;
    for b in 0..bit_len {
        for p_from in 0..n {
            if dp[b][p_from] == DUMMY {
                continue;
            }

            for p_to in 0..n {
                let bit_next = b | (1 << p_to);
                // コストは単調増加だからあってもなくても同じ
                // if bit_next == b {
                //     continue;
                // }
                let dist_cur = (xyn[p_from].0 - xyn[p_to].0).abs()
                    + (xyn[p_from].1 - xyn[p_to].1).abs()
                    + 0.max(xyn[p_to].2 - xyn[p_from].2);
                dp[bit_next][p_to] = dp[bit_next][p_to].min(dp[b][p_from] + dist_cur);
            }
        }
    }

    let mut ans = DUMMY;
    for i in 1..n {
        let dist_cur =
            (xyn[0].0 - xyn[i].0).abs() + (xyn[0].1 - xyn[i].1).abs() + 0.max(xyn[0].2 - xyn[i].2);
        ans = ans.min(dp[bit_len - 1][i] + dist_cur);
    }

    println!("{ans}");
}
