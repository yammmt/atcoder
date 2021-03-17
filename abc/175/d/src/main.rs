// 49min 1WA (33min)
// WA: ループ端数を増やし忘れ これは引っ掛かる

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        pn: [usize; n],
        cn: [i64; n],
    }

    // 自身のマスから始めて一サイクルでどれだけ増えるか/最大範囲はどこか
    // 一サイクルが正 => 回せるだけ回って残りは最大値になる範囲まで動く
    // 一サイクルが負 => 負の中で最大となる範囲まで一度だけ動く
    let mut ans = std::i64::MIN / 2;
    for i in 0..n {
        // println!("i: {}", i);
        // i 始点
        let mut cur_i = i;
        let mut cycle_len = 0;
        let mut pts_total = 0;
        while cycle_len < k && (cycle_len < 1 || cur_i != i) {
            cur_i = pn[cur_i] - 1;
            pts_total += cn[cur_i];
            ans = ans.max(pts_total);
            cycle_len += 1;
        }
        // println!("{}", pts_total);

        if cycle_len < k && pts_total > 0 {
            // 回せるだけループ
            // 一周手前から見ないと -5 => 7 => -1 のようなループで WA しそう
            let mut cur_i = i;
            let loop_num = k / cycle_len - 1;
            let mut pts_cur = pts_total * loop_num;
            ans = ans.max(pts_cur);
            for _ in 0..k % cycle_len + cycle_len {
                cur_i = pn[cur_i] - 1;
                pts_cur += cn[cur_i];
                ans = ans.max(pts_cur);
            }
            // println!("  {}", pts_cur);
        }
        // println!("{}", ans);
    }
    println!("{}", ans);
}
