// 解けてしまった黄色 桁 DP

// k を超えた数も後ろに 0 が入れば数える対象になる
// 後ろ i 桁の一箇所以上に 0 が入る => 10^i - p^i
// 99 1 => 11

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }

    let mut vn = vec![];
    while n > 0 {
        vn.push(n % 10);
        n /= 10;
    }
    vn.reverse();
    let vn = vn;
    // println!("{:?}", vn);

    // N 以下となることが確定した集団を HashMap で管理する
    // 配列では範囲が広すぎて TLE
    let mut less_n = HashMap::new();
    let mut same_n = 1;
    let mut with_zero = 0;
    for (i, nn) in vn.iter().enumerate() {
        let mut cur_less_n = HashMap::new();
        for (key, v) in &less_n {
            // ここまでに積が k となる数が v 個
            for j in 0..10 {
                let cur = key * j;
                if cur > k {
                    let keta_left = (vn.len() - i - 1) as u32;
                    with_zero += v * (10u64.pow(keta_left) - 9u64.pow(keta_left));
                    continue;
                }

                let cnt = cur_less_n.entry(cur).or_insert(0);
                *cnt += v;
            }
        }
        if i != 0 {
            // 上位桁が n と同じで今の桁で n 未満が確定するもの
            for j in 0..*nn {
                let cur = same_n * j;
                if cur > k {
                    let keta_left = (vn.len() - i - 1) as u32;
                    with_zero += 10u64.pow(keta_left) - 9u64.pow(keta_left);
                    continue;
                }

                let cnt = cur_less_n.entry(cur).or_insert(0);
                *cnt += 1;
            }
        }

        // その桁が最上位桁となるパターン
        if i == 0 {
            for j in 1..*nn {
                if j > k {
                    let keta_left = (vn.len() - i - 1) as u32;
                    with_zero += 10u64.pow(keta_left) - 9u64.pow(keta_left);
                    continue;
                }

                let cnt = cur_less_n.entry(j).or_insert(0);
                *cnt += 1;
            }
        } else {
            // 上位桁は 0 で固定されており必ず N 以下
            for j in 1..10 {
                if j > k {
                    let keta_left = (vn.len() - i - 1) as u32;
                    with_zero += 10u64.pow(keta_left) - 9u64.pow(keta_left);
                    continue;
                }

                let cnt = cur_less_n.entry(j).or_insert(0);
                *cnt += 1;
            }
        }

        // println!("{:?}", cur_less_n);
        // println!("0: {}", with_zero);
        same_n *= *nn;
        less_n = cur_less_n;
    }

    // let mut ans = less_n.values().sum::<usize>() as u64 + with_zero;
    let mut ans = with_zero;
    for a in less_n.values() {
        ans += *a;
    }
    if same_n <= k {
        ans += 1;
    }
    println!("{}", ans);
}
