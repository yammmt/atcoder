// A 問題？

use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        x: i32,
    }

    let mut tkhs = 0;
    let mut aoki = 0;
    let mut tkhs_walk = true;
    let mut aoki_walk = true;
    let mut tkhs_left = a;
    let mut aoki_left = d;
    for _ in 0..x {
        if tkhs_walk {
            tkhs += b;
        }
        tkhs_left -= 1;
        if tkhs_left == 0 {
            tkhs_left = if tkhs_walk { c } else { a };
            tkhs_walk = !tkhs_walk;
        }

        if aoki_walk {
            aoki += e;
        }
        aoki_left -= 1;
        if aoki_left == 0 {
            aoki_left = if aoki_walk { f } else { d };
            aoki_walk = !aoki_walk;
        }
    }
    // println!("{} {}", tkhs, aoki);

    println!(
        "{}",
        match tkhs.cmp(&aoki) {
            Ordering::Greater => "Takahashi",
            Ordering::Equal => "Draw",
            Ordering::Less => "Aoki",
        }
    );
}
