// -*- coding:utf-8-unix -*-

// 30min. でサンプル 4 以外

use proconio::input;

fn ncr(n: u64, r: u64) -> u64 {
    if r == 1 || n - r == 1 {
        return n;
    } else if n == r {
        return 1;
    } else {
        return ncr(n - 1, r - 1) + ncr(n - 1, r);
    }
}

fn main() {
    input! {
        n: usize,
        p: u64,
        an: [u64; n],
    }

    let even_num = an.iter().filter(|&i| *i % 2 == 0).count();
    let odd_num = n - even_num;
    let ecmb = 2u64.pow(even_num as u32);
    let mut ans = 0;

    if p == 0 {
        let mut ocmb = 0;
        let mut used_odd_num = 0;
        while used_odd_num <= odd_num {
            if used_odd_num == 0 {
                ocmb += 1;
                used_odd_num += 2;
                continue;
            }

            ocmb += ncr(odd_num as u64, used_odd_num as u64);
            used_odd_num += 2;
        }
        ans = ecmb * ocmb;
    } else {
        let mut ocmb = 0;
        let mut used_odd_num = 1;
        while used_odd_num <= odd_num {
            ocmb += ncr(odd_num as u64, used_odd_num as u64);
            used_odd_num += 2;
        }
        ans = ecmb * ocmb;
    }
    println!("{}", ans);
}
