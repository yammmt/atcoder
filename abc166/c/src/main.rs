// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
        h: [u64; n],
        ab: [(u64, u64); m],
    }

    let mut good = Vec::with_capacity(n as usize);
    for _i in 0..n {
        good.push(true);
    }

    for pass in &ab {
        if h[(pass.0 - 1) as usize] > h[(pass.1 - 1) as usize] {
            good[(pass.1 - 1) as usize] = false;
        } else if h[(pass.0 - 1) as usize] < h[(pass.1 - 1) as usize] {
            good[(pass.0 - 1) as usize] = false;
        } else {
            good[(pass.0 - 1) as usize] = false;
            good[(pass.1 - 1) as usize] = false;
        }
    }

    println!("{}", good.into_iter().filter(|v| *v == true).count());
}
