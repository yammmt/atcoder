// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let left = (xy[k].0 - xy[i].0) * (xy[j].1 - xy[i].1);
                let right = (xy[k].1 - xy[i].1) * (xy[j].0 - xy[i].0);
                if left == right {
                    println!("Yes");
                    return;
                }

                // let dij = (xy[i].0-xy[j].0).pow(2) + (xy[i].1-xy[j].1).pow(2);
                // let djk = (xy[k].0-xy[j].0).pow(2) + (xy[k].1-xy[j].1).pow(2);
                // let dki = (xy[i].0-xy[k].0).pow(2) + (xy[i].1-xy[k].1).pow(2);
                // if dij + djk == dki || dij + dki == djk || djk + dki == dij {
                //     println!("{} {} {}", i, j, k);
                //     println!("Yes");
                //     return;
                // }
            }
        }
    }

    println!("No");
}
