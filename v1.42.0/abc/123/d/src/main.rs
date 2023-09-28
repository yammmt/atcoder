// 14.5min
// 想定解が沢山ある

use proconio::input;
use std::collections::BinaryHeap;
use std::collections::HashSet;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        k: usize,
        mut ax: [i64; x],
        mut by: [i64; y],
        mut cz: [i64; z],
    }
    ax.sort();
    by.sort();
    cz.sort();
    ax.reverse();
    by.reverse();
    cz.reverse();

    let mut bh = BinaryHeap::new();
    let mut hs = HashSet::new();
    bh.push((ax[0] + by[0] + cz[0], (0, 0, 0)));
    hs.insert((0, 0, 0));
    for _ in 0..k {
        let cur = bh.pop().unwrap();
        println!("{}", cur.0);

        let x_now = (cur.1).0;
        let y_now = (cur.1).1;
        let z_now = (cur.1).2;
        if x_now < x - 1 && !hs.contains(&(x_now + 1, y_now, z_now)) {
            bh.push((ax[x_now + 1] + by[y_now] + cz[z_now], (x_now + 1, y_now, z_now)));
            hs.insert((x_now + 1, y_now, z_now));
        }
        if y_now < y - 1 && !hs.contains(&(x_now, y_now + 1, z_now)) {
            bh.push((ax[x_now] + by[y_now + 1] + cz[z_now], (x_now, y_now + 1, z_now)));
            hs.insert((x_now, y_now + 1, z_now));
        }
        if z_now < z - 1 && !hs.contains(&(x_now, y_now, z_now + 1)) {
            bh.push((ax[x_now] + by[y_now] + cz[z_now + 1], (x_now, y_now, z_now + 1)));
            hs.insert((x_now, y_now, z_now + 1));
        }
    }
}
