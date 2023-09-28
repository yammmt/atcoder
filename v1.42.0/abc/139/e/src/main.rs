// 重実装

use proconio::input;

fn main() {
    input! {
        n: usize,
        ann1: [[usize; n - 1]; n],
    }

    let mut a_idx = vec![0; n];
    let mut ans = 0;
    let mut battle_candidates = (0..n).collect::<Vec<usize>>();
    while !battle_candidates.is_empty() {
        let mut next_candidates = vec![];
        // println!("day {}", ans);
        let mut battled = vec![false; n];
        for j in &battle_candidates {
            // println!("  j: {}", j);
            if battled[*j] || a_idx[*j] == n - 1 {
                continue;
            }

            // println!("  a_idx[j]: {}", a_idx[j]);
            let enemy_idx = ann1[*j][a_idx[*j]] - 1;
            if !battled[enemy_idx]
                && a_idx[enemy_idx] < n - 1
                && ann1[enemy_idx][a_idx[enemy_idx]] - 1 == *j
            {
                a_idx[*j] += 1;
                a_idx[enemy_idx] += 1;
                battled[*j] = true;
                battled[enemy_idx] = true;
                next_candidates.push(*j);
                next_candidates.push(enemy_idx);
            }
        }

        ans += 1;
        battle_candidates = next_candidates;
    }

    println!(
        "{}",
        if a_idx.iter().all(|&a| a == n - 1) {
            ans - 1
        } else {
            -1
        }
    );
}
