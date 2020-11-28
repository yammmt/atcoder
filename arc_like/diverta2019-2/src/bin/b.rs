use proconio::input;
use std::collections::HashSet;

// 入力には x/y 座標が両方とも同じ点は含まれないため
// ある一点からコストのない一点は一点のみ存在するかしないかになる

fn main() {
    input! {
        n: usize,
        xyn: [(i64, i64); n],
    }

    let mut possible_pq = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j || (xyn[j].0 == xyn[i].0 && xyn[j].1 == xyn[i].1) {
                continue;
            }

            possible_pq.insert((xyn[j].0 - xyn[i].0, xyn[j].1 - xyn[i].1));
        }
    }

    let mut ans = n;
    // pq 固定
    for pq in &possible_pq {
        let mut cur = n;
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                if xyn[j].0 - xyn[i].0 == pq.0 && xyn[j].1 - xyn[i].1 == pq.1 {
                    cur -= 1;
                }
            }
        ans = ans.min(cur);
        }
    }
    println!("{}", ans);
}
