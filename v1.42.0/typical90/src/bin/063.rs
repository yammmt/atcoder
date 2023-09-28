// 24min
// :fu: 21-06 解けるけれどもたつく

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        phw: [[usize; w]; h],
    }

    // h を選ぶ選ばない全通り -> 列全通り試して数字一致したら保存
    let mut ans = 0;
    for bit in 0..2u64.pow(h as u32) {
        let mut used = vec![false; h];
        for i in 0..h {
            if (bit >> i) & 1 > 0 {
                used[i] = true;
            }
        }

        // println!("{:?}", used);
        let mut used_i = vec![];
        for (i, u) in used.iter().enumerate() {
            if *u {
                used_i.push(i);
            }
        }
        if used_i.is_empty() {
            continue;
        }

        let mut hm = HashMap::new();
        for j in 0..w {
            let mut pass = true;
            let mut cur_p = None;
            for i in &used_i {
                if cur_p.is_none() {
                    cur_p = Some(phw[*i][j]);
                }

                // println!("  phw[{}][{}] = {}", i, j, phw[*i][j]);
                if cur_p != Some(phw[*i][j]) {
                    pass = false;
                    break;
                }
            }

            if pass {
                let cnt = hm.entry(phw[used_i[0]][j]).or_insert(0);
                *cnt += 1;
            }
        }

        for v in hm.values() {
            ans = ans.max(used_i.len() * v);
        }
    }

    println!("{}", ans);
}
