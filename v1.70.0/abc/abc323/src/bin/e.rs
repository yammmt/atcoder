use ac_library::ModInt998244353 as Mint;
use proconio::fastout;
use proconio::input;

// mod 確率でありデバッグがし辛い

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        tn: [usize; n],
    }

    let mi0 = Mint::from(0);
    let ninv = Mint::new(1) / Mint::new(n);

    // dp[i]: 時刻 i に曲が切り替わる確率
    //        曲の切り替わるタイミングで次の曲を流す. 流した曲の終了地点が x を超えていて, かつ
    //        その曲が 1 (1-origin) であった場合には解に足す.
    //        時刻 x で曲が切り替わる場合までを判定する.
    // TLE 解: dp[i][j]: 時刻 i に曲 j が再生され始める確率
    //         再生される曲に興味はなく, 曲の種類まで見ると O(XNN) になってしまうため
    let mut dp = vec![mi0; x + 1];
    dp[0] = Mint::from(1);
    let mut ans = mi0;
    for i in 0..=x {
        // なくてもよい, どうせ `added_prob` が 0 になるので
        if dp[i] == mi0 {
            continue;
        }

        let added_prob = dp[i] * ninv;
        for (j, t) in tn.iter().enumerate() {
            let i_next = i + t;
            if i_next > x {
                if j == 0 {
                    ans += added_prob;
                }
            } else {
                dp[i_next] += added_prob;
            }
        }
    }

    println!("{ans}");
}
