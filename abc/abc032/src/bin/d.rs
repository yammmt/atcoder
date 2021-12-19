// x -> 35min + WA2
// :fu: 21-12 欲張り
// 古いのでテストケース非公開

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        vwn: [(usize, usize); n],
    }

    if n <= 30 {
        // データセット 1, 半分全列挙
        // (重さ, 価値)
        let mut front_score = vec![];
        for i in 0..2u32.pow(n as u32 / 2) {
            let mut cur_score = (0, 0);
            let mut cur_i = i;
            for j in 0..n / 2 {
                if cur_i % 2 == 1 {
                    cur_score.0 += vwn[j].1;
                    cur_score.1 += vwn[j].0;
                }
                cur_i /= 2;
            }
            if cur_score.0 <= w {
                front_score.push(cur_score);
            }
        }
        front_score.sort_unstable();
        front_score.dedup();

        let mut ans = 0;
        let n_rear = n - n / 2;
        for i in 0..2u32.pow(n_rear as u32) {
            let mut cur_score = (0, 0);
            let mut cur_i = i;
            for j in 0..n_rear {
                if cur_i % 2 == 1 {
                    cur_score.0 += vwn[j + n / 2].1;
                    cur_score.1 += vwn[j + n / 2].0;
                }
                cur_i /= 2;
            }

            if cur_score.0 > w {
                continue;
            }

            // 二分探索したいが総数が少ないので線形探索でも間に合う
            for fs in &front_score {
                if cur_score.0 + fs.0 > w {
                    break;
                }

                ans = ans.max(cur_score.1 + fs.1);
            }
        }
        println!("{}", ans);
    } else if vwn.iter().all(|vw| vw.1 <= 1000) {
        // データセット 2, w をインデックス
        // dp[i]: 重さ合計 w の場合の最大価値
        let mut dp = vec![0; w + 1];
        for vw in &vwn {
            let mut cur = dp.clone();
            for i in 0..dp.len() {
                let next_i = i + vw.1;
                if next_i > w {
                    break;
                }
                // 最大値だけ矛盾しなければよく i>0 && dp[i]==0 で弾かずとも良い

                cur[next_i] = cur[next_i].max(dp[i] + vw.0);
            }
            dp = cur;
        }
        println!("{}", dp.iter().max().unwrap());
    } else {
        // データセット 3, v をインデックス
        // dp[i]: 価値合計 i の場合の最小重さ
        let dummy = std::usize::MAX / 2;
        let max_value = vwn.iter().map(|vw| vw.0).sum::<usize>();
        let mut dp = vec![dummy; max_value + 1];
        dp[0] = 0;
        for vw in &vwn {
            let mut cur = dp.clone();
            for i in 0..dp.len() {
                let next_i = vw.0 + i;
                if next_i > max_value {
                    break;
                }

                cur[next_i] = cur[next_i].min(dp[i] + vw.1);
            }
            dp = cur;
        }

        for i in (0..max_value).rev() {
            if dp[i] <= w {
                println!("{}", i);
                return;
            }
        }
    }
}
