// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [u64; n],
    }
    an.sort();
    // println!("{:?}", an);

    let mut is_passed = vec![true; (an[n - 1] + 1) as usize];
    let mut ans = 0;
    for i in 0..n {
        if !is_passed[an[i] as usize] {
            continue;
        }

        let mut x = 2;
        while an[i] * x <= an[n - 1] {
            is_passed[(an[i] * x) as usize] = false;
            x += 1;
        }

        if (i > 0 && an[i] == an[i - 1]) || (i < n - 1 && an[i] == an[i + 1]) {
            continue;
        }
        // println!("{}", an[i]);
        ans += 1;
    }
    // println!("{:?}", is_passed);
    println!("{}", ans);
}
