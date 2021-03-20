use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        _b: usize,
    }
    let dir = [(1, 0), (0, 1)];

    if a == 0 {
        println!("1");
        return;
    }

    // A が置かれていると true
    let initial_mass = vec![vec![false; w]; h];

    let mut ans = 0;
    let mut vdq = VecDeque::new();
    vdq.push_back((initial_mass, (0, 0), a));
    while let Some(cur) = vdq.pop_front() {
        let mut next_hw = cur.1;
        if next_hw.1 < w - 1 {
            next_hw.1 += 1;
        } else {
            next_hw.0 += 1;
            next_hw.1 = 0;
        }
        if next_hw.0 == h {
            continue;
        }
        // println!("{:?}", cur);

        // 置かない
        vdq.push_back((cur.0.clone(), next_hw, cur.2));

        // 置いてあるので置けない
        if cur.0[(cur.1).0][(cur.1).1] {
            continue;
        }

        // 縦置き/横置き
        for d in &dir {
            let covered_h = (cur.1).0 + d.0;
            let covered_w = (cur.1).1 + d.1;
            if covered_h >= h || covered_w >= w || cur.0[covered_h][covered_w] {
                continue;
            }

            if cur.2 == 1 {
                ans += 1;
                continue;
            }

            let mut mass = (cur.0).clone();
            mass[(cur.1).0][(cur.1).1] = true;
            mass[covered_h][covered_w] = true;
            vdq.push_back((mass, next_hw, cur.2 - 1));
        }
    }

    println!("{}", ans);
}
