use proconio::fastout;
use proconio::input;
use std::collections::BinaryHeap;
use std::collections::HashMap;

const DUMMY: isize = isize::MAX / 2;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abcm: [(usize, usize, isize); m],
    }

    // エレベーターのある各階までの最短移動時間を求める
    // 最短到着時間が低い階から順に確定するとは限らない
    // エレベーター最上階までの移動時間に N 階までの移動時間を足すと答え

    // editorial: 階段の計算を事前にエレベーターに置き換えてしまえば, 以降の処理は共通化できて賢そう

    // 番兵
    let mut elevator_floors = vec![0, n + 1];
    // (目的地, 移動時間)
    let mut elevator_edges = HashMap::new();
    for (a, b, c) in &abcm {
        elevator_floors.push(*a);
        elevator_floors.push(*b);
        let v = elevator_edges.entry(a).or_insert(vec![]);
        v.push((b, c));
        let v = elevator_edges.entry(b).or_insert(vec![]);
        v.push((a, c));
    }
    elevator_floors.sort_unstable();
    elevator_floors.dedup();

    let mut shortest = HashMap::new();
    let mut heap = BinaryHeap::new();
    // (-cost, floor)
    heap.push((0, 1));
    while let Some(cur) = heap.pop() {
        let floor_cur = cur.1;
        if shortest.contains_key(&floor_cur) {
            continue;
        }

        let cost_cur = -cur.0;
        shortest.insert(floor_cur, cost_cur);

        // 番兵との兼ね合いで, エレベーター判定より先に弾く
        if floor_cur == n {
            break;
        }

        // エレベーターで移動する
        if let Some(edges) = elevator_edges.get(&floor_cur) {
            for e in edges {
                if !shortest.contains_key(e.0) {
                    // 思考停止で突っ込んでも上の重複除去に引っ掛かってくれるはずではある
                    heap.push((-(cost_cur + e.1), *e.0));
                }
            }
        }

        // 階段で前後のエレベーター階に移動する
        let mut lb = 0;
        let mut ub = elevator_floors.len();
        while ub - lb > 1 {
            let mid = (ub + lb) / 2;
            if elevator_floors[mid] > floor_cur {
                ub = mid;
            } else {
                lb = mid;
            }
        }

        // ub 初期値で抜けてくる
        if ub < elevator_floors.len() - 1 {
            let floor_upper = elevator_floors[ub];
            if !shortest.contains_key(&floor_upper) {
                heap.push((
                    -(cost_cur + (floor_upper - floor_cur) as isize),
                    floor_upper,
                ));
            }
        }
        // lb 側は現在地になるので, ひとつ下の階に読み替える
        // 0 到達は初回だけな気がするが
        if lb == 0 {
            continue;
        }

        lb -= 1;
        let floor_lower = elevator_floors[lb];
        if !shortest.contains_key(&floor_lower) {
            heap.push((
                -(cost_cur + (floor_cur - floor_lower) as isize),
                floor_lower,
            ));
        }
    }

    // 残りを階段として全始点を試す
    let mut ans = DUMMY;
    for (k, v) in shortest {
        ans = ans.min(v + (n - k) as isize);
    }

    println!("{ans}");
}
