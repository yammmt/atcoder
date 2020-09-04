// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u8,
        a: [u64; n],
    }

    let mut money: u64 = 1000;
    for i in 1..n {
        if a[i as usize] > a[(i - 1) as usize] {
            let num_stocks = money / a[(i - 1) as usize];
            money += num_stocks * (a[i as usize] - a[(i - 1) as usize])
        }
    }
    println!("{}", money);
}
