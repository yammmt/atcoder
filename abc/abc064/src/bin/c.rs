// 色の設定範囲について質問が三つも投げられているし
// 問題の内容のわりに平均ペナ/ペナ率がかなり高い
// 例 2 で八色外の色を用いているのに気付くか
// あるいは英文版だと "the eight colors above or not" と書いてあり有利か

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }
    let mut rank = vec![0; 9];
    for a in &an {
        if *a < 3200 {
            rank[*a / 400] += 1;
        } else {
            rank[8] += 1;
        }
    }
    // println!("{:?}", rank);

    let mincolor = rank.iter().take(8).filter(|&a| *a > 0).count();
    let maxcolor = mincolor + rank[8];
    println!("{} {}", mincolor.max(1), maxcolor);
}
