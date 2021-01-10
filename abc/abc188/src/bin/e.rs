use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [i64; n],
        xym: [(usize, usize); m],
    }
    let mut vconnected = vec![vec![]; n + 1];
    for xy in &xym {
        vconnected[xy.0].push(xy.1);
    }
    // println!("vconnected: {:?}", vconnected);

    // 制約を読み替えると有効閉路なし
    // トポロジカルソートして最安値と最高値記録すれば
    let mut vvisited = vec![0; n + 1];
    let mut vorder = vec![];
    for i in 1..n + 1 {
        if vvisited[i] != 0 {
            continue;
        }

        let mut vdq = VecDeque::new();
        vdq.push_back(i);
        while let Some(cur) = vdq.pop_back() {
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
    // println!("{:?}", vorder);

    let mut dp_buy = vec![std::i64::MAX / 2; n + 1];
    for i in 0..vorder.len() {
        // println!("i: {}", i);
        // println!("vorder[i]: {}", vorder[i]);
        for v in &vconnected[vorder[i]] {
            // println!("update v: {}", v);
            dp_buy[*v] = dp_buy[*v].min(an[vorder[i] - 1]).min(dp_buy[vorder[i]]);
        }
    }
    // println!("buy: {:?}", dp_buy);

    let mut ans = std::i64::MIN;
    for i in 0..n {
        ans = ans.max(an[i] - dp_buy[i + 1]);
    }

    println!("{}", ans);
}
