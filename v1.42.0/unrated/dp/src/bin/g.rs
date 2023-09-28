// 恐らくは再帰で解いたほうが早い

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        xym: [(usize, usize); m],
    }

    let mut vconnected = vec![vec![]; n + 1];
    for xy in &xym {
        vconnected[xy.0].push(xy.1);
    }
    // println!("root: {:?}", is_root);

    // トポロジカルソート
    let mut vvisited = vec![0; n + 1];
    let mut vorder = vec![];
    for i in 1..n + 1 {
        // println!("{:?}", vvisited);
        if vvisited[i] != 0 {
            continue;
        }

        let mut vdq = VecDeque::new();
        vdq.push_back(i);
        while let Some(cur) = vdq.pop_back() {
            // println!("cur: {:?}", cur);
            vvisited[cur] += 1;
            // 帰りがけ
            if vvisited[cur] == 2 {
                vorder.push(cur);
            }
            if vvisited[cur] >= 2 {
                continue;
            }

            vdq.push_back(cur);
            for v in &vconnected[cur] {
                if vvisited[*v] != 2 {
                    vdq.push_back(*v);
                }
            }
        }
    }
    vorder.reverse();
    // println!("order: {:?}", vorder);

    let mut dp = vec![0; n + 1];
    for i in 0..vorder.len() {
        // println!("fixed: {} = {}", vorder[i], dp[vorder[i]]);
        for v in &vconnected[vorder[i]] {
            dp[*v] = dp[*v].max(dp[vorder[i]] + 1);
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp.iter().max().unwrap());
}
