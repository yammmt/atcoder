use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    // 入力が厄介
    let mut edges = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {
            d: [isize; n-i-1],
        }
        for j in 0..n - i - 1 {
            edges[i][j + i + 1] = d[j];
            edges[j + i + 1][i] = d[j];
        }
    }

    // 全探索したい
    // 円順列で 16 個並べると TLE
    // 16!! なら間に合いそうではあるがようわからん
    // bitDP だと 2^16 は 65536 でしかない
    // dp[i][j]: 訪問済が i で現在地が j, 訪問済数が奇数であれば辺を貼らない
    let dp_n = 1 << n;
    let mut dp = vec![vec![-1; n]; dp_n];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }
    for bit in 1..dp_n {
        let mut bit_mut = bit;
        let mut bit_cnt = 0;
        for _ in 0..n {
            if bit_mut % 2 == 1 {
                bit_cnt += 1;
            }
            bit_mut /= 2;
        }

        for p_from in 0..n {
            if dp[bit][p_from] == -1 {
                // 存在しない出発点判定はどかせない？
                continue;
            }

            for p_to in 0..n {
                let bit_next = bit | (1 << p_to);
                if bit_next == bit {
                    // これがないと多重辺を貼って値が大きくなる
                    continue;
                }

                if bit_cnt % 2 == 0 {
                    dp[bit_next][p_to] = dp[bit_next][p_to].max(dp[bit][p_from]);
                } else {
                    dp[bit_next][p_to] =
                        dp[bit_next][p_to].max(dp[bit][p_from] + edges[p_from][p_to]);
                }
            }
        }
    }

    println!("{}", dp[dp_n - 1].iter().max().unwrap());
}
