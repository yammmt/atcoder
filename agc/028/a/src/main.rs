// -*- coding:utf-8-unix -*-

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64, g: u64) -> u64 {
    a / g * b
}

fn main() {
    input! {
        n: u64,
        m: u64,
        s: String,
        t: String,
    }

    let vs: Vec<char> = s.chars().collect();
    let vt: Vec<char> = t.chars().collect();
    let g = gcd(n, m);
    let l = lcm(n, m, g);
    for i in 0..g {
        if vs[(i * (n / g)) as usize] != vt[(i * (m / g)) as usize] {
            println!("-1");
            return;
        }
    }

    println!("{}", l);
}
