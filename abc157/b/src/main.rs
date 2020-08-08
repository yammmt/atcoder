// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: [[u8; 3]; 3],
        n: u8,
        b: [u8; n],
    }

    if (b.contains(&a[0][0]) && b.contains(&a[0][1]) && b.contains(&a[0][2]))
        || (b.contains(&a[1][0]) && b.contains(&a[1][1]) && b.contains(&a[1][2]))
        || (b.contains(&a[2][0]) && b.contains(&a[2][1]) && b.contains(&a[2][2]))
        || (b.contains(&a[0][0]) && b.contains(&a[1][0]) && b.contains(&a[2][0]))
        || (b.contains(&a[0][1]) && b.contains(&a[1][1]) && b.contains(&a[2][1]))
        || (b.contains(&a[0][2]) && b.contains(&a[1][2]) && b.contains(&a[2][2]))
        || (b.contains(&a[0][0]) && b.contains(&a[1][1]) && b.contains(&a[2][2]))
        || (b.contains(&a[2][0]) && b.contains(&a[1][1]) && b.contains(&a[0][2]))
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
