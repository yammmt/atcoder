use proconio::fastout;
use proconio::input;

fn win(me: f64, opponent: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((opponent - me) / 400.0))
}

#[fastout]
fn main() {
    input! {
        k: usize,
    }
    let member_num = 2u32.pow(k as u32) as usize;
    input! {
        // 入力が整数とは書いていない, 記載漏れっぽい気はするけど
        r2k: [f64; member_num],
    }

    // 勝ち残っている確率
    let mut dp = vec![1.0; member_num];
    for i in 0..k {
        let mut cur = vec![0.0; member_num];
        let mut j = 0;
        // トーナメント片側で勝ち残っている可能性のある人の数
        let candidate_num = 2u32.pow(i as u32) as usize;
        while j < member_num {
            for jj in 0..candidate_num {
                let p = j + jj;
                for jjj in 0..candidate_num {
                    let q = j + candidate_num + jjj;
                    // println!("  {p} - {q}");
                    let p_win = win(r2k[p], r2k[q]) * dp[p] * dp[q];
                    let q_win = win(r2k[q], r2k[p]) * dp[q] * dp[p];
                    cur[p] += p_win;
                    cur[q] += q_win;
                }
            }
            j += candidate_num * 2;
        }
        dp = cur;
    }

    for d in dp {
        // 絶対誤差 10^(-6) 以内
        println!("{:.8}", d);
    }
}
