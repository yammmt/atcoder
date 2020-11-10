// -*- coding:utf-8-unix -*-

// 30min (非模範解)

use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut pa: [u64; a],
        mut qb: [u64; b],
        mut rc: [u64; c],
    }
    pa.sort();
    qb.sort();
    rc.sort();
    pa.reverse();
    qb.reverse();
    rc.reverse();

    let mut vans = vec![];
    vans.append(&mut pa[0..x].to_vec());
    vans.append(&mut qb[0..y].to_vec());
    vans.append(&mut rc);
    vans.sort();
    vans.reverse();
    let ans = vans.iter().take(x + y).fold(0, |a, b| a + *b);
    println!("{}", ans);
}
