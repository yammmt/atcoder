// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut abc: [u8; 3],
    }

    abc.sort();
    let mut ans = 0;
    if abc.iter().all(|&x| x % 2 == 0) || abc.iter().all(|&x| x % 2 == 1) {
        ans = (abc[2] - abc[0]) / 2 + (abc[2] - abc[1]) / 2;
    } else {
        if abc[0] % 2 == abc[1] % 2 {
            abc[0] += 1;
            abc[1] += 1;
        } else if abc[0] % 2 == abc[2] % 2 {
            abc[0] += 1;
            abc[2] += 1;
        } else {
            abc[1] += 1;
            abc[2] += 1;
        }
        ans = (abc[2] - abc[0]) / 2 + (abc[2] - abc[1]) / 2 + 1;
    }
    println!("{}", ans);
}
