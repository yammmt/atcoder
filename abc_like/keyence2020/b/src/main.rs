// -*- coding:utf-8-unix -*-

// 32min 1WA

use proconio::input;

fn main() {
    input! {
        n: usize,
        xln: [(i64, i64); n],
    }

    let mut mvarea = Vec::with_capacity(n);
    for i in &xln {
        // (right, left)
        mvarea.push((i.0 + i.1, i.0 - i.1));
    }
    mvarea.sort();

    let mut ans = 0;
    let mut cright = std::i64::MIN;
    for i in 0..n {
        if cright > mvarea[i].1 {
            continue;
        }

        cright = mvarea[i].0;
        ans += 1;
    }
    println!("{}", ans);
}
