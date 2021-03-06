// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let mut ans = 0;
    for ab in 2..2 * n + 1 {
        let cd = ab - k;
        if !(cd >= 2 && cd <= 2 * n) {
            continue;
        }

        // 公式の解説 2N + 1 - K: 作る数 K に対し (a, b) = (K/2, K/2) を一つ考える (+1)
        // N の範囲 (上限) が 1 増えるごとに (a + 1, b - 1), (a - 1, b + 1) の 2 通りずつパターンが増える
        // N の範囲の上昇幅は N - K/2 で与えられるので, a + b = K となる (a, b) の組み合わせの個数は
        // 1 + 2 * (N - K/2) = 2N + 1 - K  と変式できる
        // K/2 があるが K の偶奇は問わず切り捨てで良い
        // K が奇数の場合 (1 - K) が偶数となり (a, b) の組み合わせは偶数個になる

        // ab = 6 を考えると (a, b) = (1, 5), (2, 4), (3, 3), (4, 2), (5, 1) の五通り
        // N が 5 以上であればそのまま全通り (ab - 1 通り) 列挙できるが N = 4 で二通り減って
        // N = 3 でもう二通り減る => N が (ab - 1) から 1 小さくなるごとに組み合わせが 2 ずつ減る
        // ab/cd 範囲外は既に除外されているので最低一通りは見つかる (上の例で N = 2 は既に弾かれている)
        let abnum = (ab - 1) - 2 * (ab - 1 - n).max(0);
        let cdnum = (cd - 1) - 2 * (cd - 1 - n).max(0);
        // println!("ab: {}, cd: {}", ab, cd);
        // println!("abnum: {}, cdnum: {}", abnum, cdnum);
        ans += abnum * cdnum;
    }
    println!("{}", ans);
}
