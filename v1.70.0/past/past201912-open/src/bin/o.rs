// https://wk1080id.hatenablog.com/entry/2020/04/25/122406

use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        an6: [[usize; 6]; n],
    }

    // 座圧
    // 初手, サイコロを振っていない状態を 0 で固定するため,
    // a_i として登場しない十分小さい数 (0) を入れておく
    let mut v = vec![0];
    for an in &an6 {
        for &a in an {
            v.push(a);
        }
    }
    v.sort_unstable();
    v.dedup();
    let mut dice_num_to_idx = HashMap::new();
    for (i, &d) in v.iter().enumerate() {
        dice_num_to_idx.insert(d, i);
    }
    let dl = dice_num_to_idx.len();

    // 出目 -> それを含むサイコロ
    // 重複は除去してはならない…のだが, 制約より重複は存在しない
    let mut num_to_dices = vec![vec![]; dl];
    for (i, an) in an6.iter().enumerate() {
        for a in an {
            let di = *dice_num_to_idx.get(a).unwrap();
            num_to_dices[di].push(i);
        }
    }

    let mut dp = vec![0.0f64; dl];
    let mut dices = vec![0.0f64; n];
    let mut dices_max = 0.0;
    for j in (0..dl).rev() {
        // 今回最適なサイコロを降る + 最適なサイコロのその後の期待値
        dp[j] = 1.0 + dices_max;

        // 自身が寄与するサイコロの期待値を更新する
        for &d in &num_to_dices[j] {
            dices[d] += dp[j] / 6.0;
            dices_max = dices_max.max(dices[d]);
        }
    }
    // println!("dp: {:?}", dp);
    // println!("dices: {:?}", dices);

    println!("{}", dp[0]);
}
