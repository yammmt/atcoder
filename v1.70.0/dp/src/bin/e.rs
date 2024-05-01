use proconio::fastout;
use proconio::input;

const DUMMY: usize = usize::MAX / 3;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wvn: [(usize, usize); n],
    }
    let vsum = wvn.iter().map(|wv| wv.1).sum::<usize>();

    // 価値を達成するために必要な最低容量
    let mut dp = vec![DUMMY; vsum + 1];
    dp[0] = 0;
    for (w, v) in wvn {
        let mut cur = vec![DUMMY; vsum + 1];
        for i in 0..vsum {
            // 選ばない
            cur[i] = cur[i].min(dp[i]);

            // 選ぶ
            let v_nxt = i + v;
            if v_nxt > vsum {
                continue;
            }

            cur[v_nxt] = cur[v_nxt].min(dp[i] + w);
        }
        dp = cur;
    }

    let mut ans = 0;
    for (i, &v) in dp.iter().enumerate() {
        if v <= w {
            ans = i;
        }
    }

    println!("{ans}");
}
