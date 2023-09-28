// :fu: 21-08 bitDP つらい

use proconio::input;
use proconio::marker::Bytes;

const BIT_LEN_MAX: usize = 1 << 10;

fn main() {
    input! {
        _n: usize,
        s: Bytes,
    }
    let d = 998_244_353;

    // dp[i][j]: 出場したコンテストの集合が i であって最後に出場したものが j
    let mut dp = vec![vec![0; 11]; BIT_LEN_MAX];
    dp[0][0] = 1;
    for c in &s {
        let cur_byte = (*c - b'A' + 1) as usize;
        let mut cur = vec![vec![0; 11]; BIT_LEN_MAX];
        for i in 0..BIT_LEN_MAX {
            for j in 0..11 {
                if dp[i][j] == 0 {
                    continue;
                }

                cur[i][j] = (cur[i][j] + dp[i][j]) % d;

                // 最後に出たものと同じ or まだ出ていないなら付け足しても良い
                if cur_byte == j || i & (1 << (cur_byte - 1)) == 0 {
                    let next_i = i | (1 << (cur_byte - 1));
                    cur[next_i][cur_byte] = (cur[next_i][cur_byte] + dp[i][j]) % d;
                }
            }
        }

        dp = cur;
    }

    // "1 個以上選ぶ"
    dp[0][0] = 0;
    let mut ans = 0u64;
    dp.iter().for_each(|v| ans = (ans + v.iter().sum::<u64>()) % d);
    println!("{}", ans);
}
