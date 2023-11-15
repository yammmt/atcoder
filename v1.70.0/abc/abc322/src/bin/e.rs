use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
    }
    let mut c = vec![];
    let mut ank = vec![];
    for _ in 0..n {
        input! {
            cc: usize,
            an: [usize; k],
        }
        c.push(cc);
        ank.push(an);
    }

    // K, P <= 5 より個々の p が 5 以上となったらスルーできる
    // N = 100 個の開発案について,
    // dp[i][a][b][c][d][e]: i 個目の開発案まで見て
    // パラメーター [a][b][c][d][e] を達成する最小コスト, としても
    // 計算量はオーダーのカッコ内が 100x5x5x5x5x5=2500 程度におさまるはず
    // N が不定数で書きにくい, HashMap にする

    // (vec![param], cost)
    let mut hm_cur = HashMap::new();
    let v = vec![0; k];
    hm_cur.insert(v, 0);
    for i in 0..n {
        let mut hm_nxt: HashMap<Vec<usize>, usize> = HashMap::new();
        for (key, &val) in hm_cur.iter() {
            // 選択しなければそのまま
            let cheaper = if let Some(&cur) = hm_nxt.get(key) {
                cur.min(val)
            } else {
                val
            };
            hm_nxt.insert(key.to_vec(), cheaper);

            // 選択する
            let mut key_nxt = key.to_vec();
            for j in 0..k {
                key_nxt[j] = (key[j] + ank[i][j]).min(p);
            }
            let cheaper = if let Some(&cur) = hm_nxt.get(&key_nxt) {
                cur.min(val + c[i])
            } else {
                val + c[i]
            };
            hm_nxt.insert(key_nxt, cheaper);
        }
        hm_cur = hm_nxt;
    }
    // println!("{:?}", hm_cur);

    let v = vec![p; k];

    println!(
        "{}",
        if let Some(val) = hm_cur.get(&v) {
            *val as isize
        } else {
            -1
        }
    );
}
