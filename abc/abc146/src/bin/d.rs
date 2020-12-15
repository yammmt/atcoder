// https://drken1215.hatenablog.com/entry/2020/04/26/172200

use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n - 1],
    }

    let mut vedges = vec![vec![]; n];
    for i in 0..n - 1 {
        vedges[abn[i].0 - 1].push(abn[i].1 - 1);
        vedges[abn[i].1 - 1].push(abn[i].0 - 1);
    }
    // println!("vedges: {:?}", vedges);

    let mut colornum = 0;
    let mut start_from = n + 2;
    for i in 0..vedges.len() {
        if vedges[i].len() > colornum {
            colornum = vedges[i].len();
            start_from = i;
        }
    }

    let mut vdq = VecDeque::new();
    let mut vans = HashMap::new();
    for i in 0..vedges[start_from].len() {
        vans.insert((start_from, vedges[start_from][i]), i + 1);
        vans.insert((vedges[start_from][i], start_from), i + 1);
        // child, parent_color
        vdq.push_back((vedges[start_from][i], i + 1));
    }

    while let Some(cur) = vdq.pop_front() {
        let mut color = 1;
        while let Some(e) = vedges[cur.0].pop() {
            if vans.contains_key(&(cur.0, e)) {
                continue;
            }

            if color == cur.1 {
                color += 1;
            }
            vans.insert((cur.0, e), color);
            vans.insert((e, cur.0), color);
            vdq.push_back((e, color));
            color += 1;
        }
    }

    println!("{}", colornum);
    for i in 0..n - 1 {
        println!("{}", vans.get(&(abn[i].0 - 1, abn[i].1 - 1)).unwrap());
    }
}
