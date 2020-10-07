// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
    }
    let mut v = vec![];
    for i in 0..m {
        input! {
            p: usize,
            y: usize,
        }
        v.push((p, y, i));
    }

    v.sort_by(|a, b| {
        if a.0 != b.0 {
            a.cmp(b)
        } else {
            a.1.cmp(&b.1)
        }
    });
    let mut vans = vec![String::default(); m];
    let mut idx_in_p = 1;
    for i in 0..m {
        if i > 0 && v[i].0 != v[i - 1].0 {
            idx_in_p = 1;
        }
        vans[v[i].2] = format!("{:06}{:06}", v[i].0, idx_in_p);
        idx_in_p += 1;
    }

    for i in &vans {
        println!("{}", i);
    }
}
