// 実装が楽そう:
// https://twitter.com/kyopro_friends/status/1294632533582200832

use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        pn: [Usize1; n],
        cn: [i64; n],
    }

    let move_next = |i: &mut usize, score: &mut i64, ans: &mut i64| {
        *i = pn[*i];
        *score += cn[*i];
        *ans = *ans.max(score);
    };

    let mut ans = -1_000_000_001;
    for i in 0..n {
        let mut cur_i = i;
        let mut score = 0;
        let mut visited = vec![false; n];
        let mut cycle_len = 0;
        for _ in 0..k.min(n) {
            if visited[cur_i] {
                break;
            }

            visited[cur_i] = true;
            move_next(&mut cur_i, &mut score, &mut ans);
            cycle_len += 1;
        }

        if score >= 0 {
            // 回れるだけ回って最後を総当り
            // -1: 回りきらないほうがよい場合がある,
            //     例えば得点が [-1, -1, 100, -1] の順に得られるなら余り 1 となると
            //     最高点を取れない
            let cycle_cnt = k / cycle_len - 1;
            score *= cycle_cnt as i64;
            cur_i = i;
            for _ in 0..k - cycle_len * cycle_cnt {
                move_next(&mut cur_i, &mut score, &mut ans);
            }
        }
        // else は不要, サイクル内で随時最大値を更新したため
    }

    println!("{ans}");
}
