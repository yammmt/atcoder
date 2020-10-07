// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        k: u32,
        s: u32,
    }

    let mut ans = 0;
    for x in 0..k + 1{
        // println!("x: {}", x);
        if x > s {
            break;
        }

        let yz = s - x;
        // println!("y + z = {}", yz);
        if yz <= k {
            ans += yz + 1;
        } else if yz <= 2 * k{
            ans += (yz + 1) - 2 * (yz - k);
        }
        // println!("ans: {}", ans);
    }
    println!("{}", ans);
}
