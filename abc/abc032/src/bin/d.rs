// :fu: :fu: 21-11 欲張り この実装量と時期でこの程度の diff によくもおさまったというお気持ち
// 古いのでテストケース非公開
// RE: 価値をインデックスとして要素外参照
// WA: 問題文を読めていない

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        vwn: [(usize, usize); n],
    }

    let vsum = vwn.iter().map(|vw| vw.0).sum::<usize>();
    let wsum = vwn.iter().map(|vw| vw.1).sum::<usize>();

    if vwn.iter().all(|vw| vw.1 <= 1000) {
        // dp[i]: 重さ i 内での最大価値
        let mut dp = vec![None; wsum + 1];
        dp[0] = Some(0);
        for vw in &vwn {
            let mut cur = dp.clone();
            for i in 0..wsum + 1 {
                if dp[i].is_none() {
                    continue;
                }

                let next_w = i + vw.1;
                let next_v = dp[i].unwrap() + vw.0;
                if next_w > wsum.min(w) {
                    continue;
                }

                cur[next_w] = if cur[next_w].is_some() {
                    Some(cur[next_w].unwrap().max(next_v))
                } else {
                    Some(next_v)
                };
            }
            // println!("{:?}", cur);
            dp = cur;
        }

        let mut ans = 0;
        dp.iter()
            .take(wsum.min(w) + 1)
            .filter(|&d| d.is_some())
            .for_each(|&d| ans = ans.max(d.unwrap()));
        println!("{}", ans);
    } else if vwn.iter().all(|vw| vw.0 <= 1000) {
        // dp[i]: 価値 i を達成する最小重さ
        let mut dp = vec![None; vsum + 1];
        dp[0] = Some(0);
        for vw in &vwn {
            let mut cur = dp.clone();
            for i in 0..vsum + 1 {
                if dp[i].is_none() {
                    continue;
                }

                let next_v = i + vw.0;
                let next_w = dp[i].unwrap() + vw.1;
                if next_w > w {
                    continue;
                }

                // next_v <= vsum は保証される
                cur[next_v] = if cur[next_v].is_some() {
                    Some(cur[next_v].unwrap().min(next_w))
                } else {
                    Some(next_w)
                };
            }
            dp = cur;
        }

        for i in (0..vsum + 1).rev() {
            if dp[i].is_some() {
                println!("{}", i);
                return;
            }
        }
    } else {
        // AC
        // n <= 30
        let front = vwn
            .iter()
            .take(n / 2)
            .copied()
            .collect::<Vec<(usize, usize)>>();
        let rear = vwn
            .iter()
            .skip(n / 2)
            .copied()
            .collect::<Vec<(usize, usize)>>();

        // front に対して全列挙
        // (重さ, **その重さ以下の** 最大価値)
        let mut front_all = vec![];
        for i in 0..2u32.pow(front.len() as u32) {
            let mut cur_v = 0;
            let mut cur_w = 0;
            let mut ii = i;
            for f in &front {
                if ii % 2 == 1 {
                    cur_v += f.0;
                    cur_w += f.1;
                }
                ii /= 2;
            }
            if cur_w > w {
                continue;
            }

            front_all.push((cur_w, cur_v));
        }
        front_all.sort_unstable();
        front_all.dedup();
        for i in 1..front_all.len() {
            front_all[i] = (front_all[i].0, front_all[i].1.max(front_all[i - 1].1));
        }
        front_all.dedup();

        // rear に対して全列挙しつつ答えを更新
        let mut ans = 0;
        for i in 0..2u32.pow(rear.len() as u32) {
            let mut cur_v = 0;
            let mut cur_w = 0;
            let mut ii = i;
            for r in &rear {
                if ii % 2 == 1 {
                    cur_v += r.0;
                    cur_w += r.1;
                }
                ii /= 2;
            }
            if cur_w > w {
                continue;
            } else if cur_w + front_all.last().unwrap().0 <= w {
                ans = ans.max(cur_v + front_all.last().unwrap().1);
                continue;
            }

            // front 側全列挙で (0, 0) が必ず挿入されている
            let mut pass = 0;
            let mut fail = front_all.len() - 1;
            while fail - pass > 1 {
                let mid = (pass + fail) / 2;
                if cur_w + front_all[mid].0 <= w {
                    pass = mid;
                } else {
                    fail = mid;
                }
            }
            ans = ans.max(cur_v + front_all[pass].1);
        }

        println!("{}", ans);
    }
}
