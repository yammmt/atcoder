use proconio::input;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        m: usize,
        uvm: [(usize, usize); m],
        p8: [usize; 8],
    }

    let mut edges = vec![vec![]; 9];
    for uv in &uvm {
        edges[uv.0 - 1].push(uv.1 - 1);
        edges[uv.1 - 1].push(uv.0 - 1);
    }
    let edges = edges;

    // 連結成分内に頂点が位置しない場合は明らかに不可だが, それは十分条件ではない
    // 空の頂点を移動させる方法を全探索しても間に合いそう (9! = 362,880, 実行時間も 4s)

    // どの頂点にどのコマが置かれているか
    // ついでに 1-indexed に揃える
    let mut v_map = vec![None; 9];
    for (i, p) in p8.iter().enumerate() {
        v_map[*p - 1] = Some(i);
    }
    let mut ans_map = vec![None; 9];
    for i in 0..8 {
        ans_map[i] = Some(i);
    }

    let mut vdq = VecDeque::new();
    let mut searched = HashSet::new();
    vdq.push_back((v_map.clone(), 0));
    searched.insert(v_map);
    while let Some(cur) = vdq.pop_front() {
        if cur.0 == ans_map {
            println!("{}", cur.1);
            return;
        }

        // 空マスが探索始点となる
        let mut search_from = 0;
        for (i, p) in cur.0.iter().enumerate() {
            if p.is_none() {
                search_from = i;
                break;
            }
        }

        for e in &edges[search_from] {
            let mut cur_map = cur.0.clone();
            cur_map.swap(search_from, *e);
            if searched.contains(&cur_map) {
                continue;
            }

            vdq.push_back((cur_map.clone(), cur.1 + 1));
            searched.insert(cur_map);
        }
    }

    println!("-1");
}
