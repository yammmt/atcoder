// あほした

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
    }

    let anmax = *an.iter().max().unwrap();
    let mut an_set = HashSet::new();
    an.iter().for_each(|&a| { an_set.insert(a); });

    let mut is_ans = vec![true; m + 1];
    is_ans[0] = false;
    for i in 2..m + 1 {
        if !is_ans[i] { continue; }

        // i はどれかの an[i] の因数？
        let mut pass = true;
        let mut cur = i;
        while cur <= anmax {
            if an_set.contains(&cur) {
                pass = false;
                break;
            }
            cur += i;
        }

        if !pass {
            let mut cur = i;
            while cur <= m {
                is_ans[cur] = false;
                cur += i;
            }
        }
    }
    // println!("{:?}", is_ans);

    let mut ans = vec![];
    for (i, a) in is_ans.iter().enumerate() {
        if *a {
            ans.push(i);
        }
    }

    // GCD が 1 == すべての A_i と互いに素
    println!("{}", ans.len());
    for a in &ans {
        println!("{}", a);
    }
}
