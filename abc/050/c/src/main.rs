// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * repeat_square(n, p - 1, m)) % m
    }
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    if n == 1 {
        if a[0] == 0 {
            println!("1");
        } else {
            println!("0");
        }
        return;
    }

    let divisor = 10u64.pow(9) + 7;
    let mut hm = HashMap::new();
    for i in &a {
        let cnt = hm.entry(i).or_insert(0);
        *cnt += 1;
    }

    let mut abs_diff = 0;
    if n % 2 == 0 {
        abs_diff = 1;
    } else {
        if hm.get(&0) != Some(&1) {
            println!("0");
            return;
        }

        abs_diff = 2;
    }

    loop {
        let a = abs_diff;

        if n % 2 == 0 {
            // ex.5 3 1 1 3 5
            if a % 2 == 0 {
                if hm.get(&a) != None {
                    println!("0");
                    return;
                }
            } else {
                if hm.get(&a) != Some(&2) {
                    println!("0");
                    return;
                }
            }
        } else {
            // ex. 4 2 0 2 4
            if a % 2 == 0 {
                if hm.get(&a) != Some(&2) {
                    println!("0");
                    return;
                }
            } else {
                if hm.get(&a) != None {
                    println!("0");
                    return;
                }
            }
        }

        if abs_diff == (n - 1) as u64 {
            break;
        }

        abs_diff += 1;
    }

    println!("{}", repeat_square(2, (n / 2) as u64, divisor));
}
