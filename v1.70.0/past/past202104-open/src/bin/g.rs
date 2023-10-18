// TODO: 長すぎる、やり直す
// failed:
// 3 2 2
// 1 2 1
// 2 3 10
// 1 10

use proconio::input;
use std::collections::BTreeSet;

const DUMMY: usize = usize::MAX / 2;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcm: [(usize, usize, usize); m],
        xq: [usize; q],
    }

    // i 日目に行けるようになった都市にはその後もずっといれる
    // つまりは各都市にいける最短日数を出せばよい

    // (cost, city)
    let mut edges = vec![vec![]; n];
    for abc in &abcm {
        let a = abc.0 - 1;
        let b = abc.1 - 1;
        let c = abc.2;
        edges[a].push((c, b));
        edges[b].push((c, a));
    }

    // Dijkstra
    let mut earliest_days = vec![DUMMY; n];
    let mut btm = BTreeSet::new();
    earliest_days[0] = 0;
    for v in &edges[0] {
        btm.insert(*v);
    }
    // println!("{:?}", btm);
    let mut now_day = 0;
    let mut next_day_edges = vec![];
    while let Some(cur) = btm.pop_first() {
        // println!("day: {now_day}, xq: {}, cur: {:?}", xq[now_day], cur);
        // println!("  btm: {:?}", btm);
        // println!("  nde: {:?}", next_day_edges);
        if now_day == q {
            break;
        } else if earliest_days[cur.1] != DUMMY {
            // 訪問済みで処理不要
            if btm.is_empty() {
                now_day += 1;
                for v in &next_day_edges {
                    btm.insert(*v);
                    // println!("  insert {:?}", v);
                }
                next_day_edges.clear();
            }

            continue;
        } else if cur.0 > xq[now_day] {
            // この日は最短でも到着不可なので次の日に
            // println!("  next");
            now_day += 1;
            btm.insert(cur);
            for v in &next_day_edges {
                btm.insert(*v);
                // println!("  insert {:?}", v);
            }
            next_day_edges.clear();
            continue;
        }

        // 新たに到着
        // n 日目の到着はまとめて que に入れ直さないとだめ
        earliest_days[cur.1] = now_day;
        for v in &edges[cur.1] {
            // println!("  to {:?}", v);
            if earliest_days[v.1] == DUMMY {
                // println!("    push");
                next_day_edges.push(*v);
            }
        }

        if btm.is_empty() {
            now_day += 1;
            for v in &next_day_edges {
                btm.insert(*v);
                // println!("  insert {:?}", v);
            }
            next_day_edges.clear();
        }
    }
    // println!("{:?}", earliest_days);

    // n 日目を累積和
    let mut imos_day = vec![0; q];
    for ed in &earliest_days {
        if *ed > q - 1 {
            continue;
        }

        imos_day[*ed] += 1;
    }

    let mut ans = 0;
    for d in &imos_day {
        ans += d;
        println!("{ans}");
    }
}
