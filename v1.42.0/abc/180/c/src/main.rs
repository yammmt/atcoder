// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    let mut idx = 1;
    loop {
        if idx * idx > n {
            break;
        }
        if n % idx == 0 {
            ans.push(idx);
            if idx != n / idx {
                ans.push(n / idx);
            }
        }
        idx += 1;
    }

    ans.sort();
    for i in &ans {
        println!("{}", *i);
    }
}
