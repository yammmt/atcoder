// 15min

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        m: usize,
        n: usize,
        amn: [[usize; n]; m],
    }

    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            // (i, j) 基点に全通り探索
            if amn[i][j] == 0 {
                continue;
            }

            let mut cur_stage = amn.clone();
            let mut vdq = VecDeque::new();
            cur_stage[i][j] = 0;
            vdq.push_back(((i, j), 1, cur_stage));
            while let Some(cur) = vdq.pop_back() {
                let mut can_continue = false;
                for d in &dir {
                    let inext_i = (cur.0).0 as isize + d.0;
                    let inext_j = (cur.0).1 as isize + d.1;
                    if inext_i < 0 || inext_i >= m as isize || inext_j < 0 || inext_j >= n as isize {
                        continue;
                    }

                    let unext_i = inext_i as usize;
                    let unext_j = inext_j as usize;
                    if cur.2[unext_i][unext_j] == 1 {
                        can_continue = true;
                        let mut next_stage = cur.2.clone();
                        next_stage[unext_i][unext_j] = 0;
                        vdq.push_back(((unext_i, unext_j), cur.1 + 1, next_stage));
                    }
                }

                if !can_continue {
                    ans = ans.max(cur.1);
                }
            }
        }
    }
    println!("{}", ans);
}
