// :fu: 21-09 もたつく

use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;

// 処理が違うので entry と if を組み合わせるの無理では
#[allow(clippy::map_entry)]
fn main() {
    input! {
        _n: usize,
        m: usize,
    }
    let mut a = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            ak: [usize; k],
        }
        a.push(ak);
    }

    // "ちょうど二個ずつ" なので貪欲法が通る
    // しかし探索を高速化しないと明らかに TLE する

    // 一番上にあるボールを記憶しておく
    let mut hm = HashMap::new();
    // let mut ball_pos = vec![vec![]; n];
    // let mut empty_poll_num = 0;
    let mut searched = VecDeque::new();
    (0..m).for_each(|i| {
        searched.push_back(i);
    });
    while let Some(idx) = searched.pop_front() {
        if a[idx].is_empty() {
            continue;
        }

        let cur = a[idx][a[idx].len() - 1];
        if hm.contains_key(&cur) {
            a[idx].pop();
            // 既に入っていた分は浮かし直す
            let poped_prev: usize = *hm.get(&cur).unwrap();
            hm.remove(&cur);
            a[poped_prev].pop();
            searched.push_front(poped_prev);
            // 自身の入れ直しは次ループに回す
            searched.push_front(idx);
        } else {
            hm.insert(cur, idx);
        }
    }
    // println!("{:?}", a);

    println!(
        "{}",
        if a.iter().all(|aa| aa.is_empty()) {
            "Yes"
        } else {
            "No"
        }
    );
}
