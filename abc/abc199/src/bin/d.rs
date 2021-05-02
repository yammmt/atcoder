// 青 テストケース未公開
// WA: A. 探索順を決める DFS で同じ頂点を何度も見ていた
//     B. 接続頂点数 > 2 で達成不可と判定してしまっていた (全探索走るのでそっちで弾かれる)

use proconio::input;
use std::collections::VecDeque;

// 3^20 > 10^9

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    let mut edges = vec![vec![]; n];
    for ab in &abm {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }

    // 頂点数は高々 20 個であり多少は雑な全検索でも通る
    // 探索順を決めるのと実際の色塗りを同時にやろうとするとダメそう
    // というのも探索終了判定がうまくかけない
    let mut visited = vec![false; n];
    let mut ans = 1u64;
    for i in 0..n {
        if visited[i] {
            continue;
        }

        // 探索順を決める
        let mut search_order = vec![];
        let mut vdq = VecDeque::new();
        vdq.push_back(i);
        while let Some(cur) = vdq.pop_back() {
            if visited[cur] {
                continue;
            }

            visited[cur] = true;
            search_order.push(cur);
            for e in &edges[cur] {
                if !visited[*e] {
                    vdq.push_back(*e);
                }
            }
        }
        // println!("{:?}", search_order);

        // 最初の色は三通り
        ans *= 3;
        if search_order.len() == 1 {
            continue;
        }

        let mut cur_ans = 0;
        let colors = vec![0; n];
        let mut vdq = VecDeque::new();
        vdq.push_back((0, colors));
        while let Some(cur) = vdq.pop_back() {
            if cur.0 == 0 {
                // 一点目は R に固定する (G/B 分を踏まえ既に三倍している)
                let mut cur_colors = cur.1.clone();
                cur_colors[search_order[0]] = 1;
                vdq.push_back((1, cur_colors));
            } else {
                // すべての接点を調べて使われていない色を入れて再帰 or カウント終了
                let mut color_used = vec![false; 4];
                for e in &edges[search_order[cur.0]] {
                    color_used[cur.1[*e]] = true;
                }
                for (i, cu) in color_used.iter().enumerate() {
                    if i == 0 || *cu {
                        continue;
                    }

                    if cur.0 == search_order.len() - 1 {
                        // 自身を色 i で塗って終了
                        cur_ans += 1;
                    } else {
                        // 自身を色 i で塗って次の頂点へ
                        let mut cur_colors = cur.1.clone();
                        cur_colors[search_order[cur.0]] = i;
                        vdq.push_back((cur.0 + 1, cur_colors));
                    }
                }
            }
        }

        ans *= cur_ans;
    }

    println!("{}", ans);
}
