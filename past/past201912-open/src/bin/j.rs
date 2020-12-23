// 左下 -> 右下と右下 -> 右上の二段階にしようにも永続的な盤面変化が厄介
// 全探索かけると右下到達段階で 81,445 通りあって TLE
// 予め均すマスを決め打ちしようにも 2^50 通りの選択肢があり TLE
// n 円で整備すると決め打つ？整備 0 で行けるかは一度 BFS で良く上解は全マス撤去なので二分探索にはなるが
// 結局全探索を挟むので違いそう
// 盤面変化があるので DP でもなさそう

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[usize; w]; h],
    }

    let dir = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut from_migishita = VecDeque::new();
    let mut vdq = VecDeque::new();
    let mut visited = vec![vec![false; w]; h];
    visited[h - 1][0] = true;
    vdq.push_back(((h - 1, 0), 0, ahw, visited));
    // 右下に行くまでの全通り (同じパスを二度通らない)
    while let Some(cur) = vdq.pop_back() {
        // cnt += 1;
        // println!("{:?}", cur);
        for d in &dir {
            let next_mass = ((cur.0).0 as isize + d.0, (cur.0).1 as isize + d.1);
            if next_mass.0 < 0
                || next_mass.0 > (h - 1) as isize
                || next_mass.1 < 0
                || next_mass.1 > (w - 1) as isize
                || cur.3[next_mass.0 as usize][next_mass.1 as usize]
            {
                continue;
            }

            let next_mass_u = (next_mass.0 as usize, next_mass.1 as usize);
            let mut cost = cur.1;
            let mut base_mass = cur.2.clone();
            if cur.2[next_mass_u.0][next_mass_u.1] != 0 {
                cost += base_mass[next_mass_u.0][next_mass_u.1] as u64;
                base_mass[next_mass_u.0][next_mass_u.1] = 0;
            }
            let mut next_visited = (cur.3).clone();
            next_visited[next_mass_u.0][next_mass_u.1] = true;

            // migishita?
            if next_mass_u.0 == h - 1 &&next_mass_u.1 == w - 1 {
                from_migishita.push_back((next_mass_u, cost, base_mass));
            } else {
                vdq.push_back((next_mass_u, cost, base_mass, next_visited));
            }
        }
    }
    println!("{:?}", from_migishita.len());

    let mut ans = std::u64::MAX;

    println!("{}", ans);
}
