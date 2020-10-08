// -*- coding:utf-8-unix -*-

use proconio::input;
use permutohedron::heap_recursive;
use std::collections::HashSet;

// Debug ビルド (通常の `cargo test`) と Release ビルドでかなりの速度差がある

fn num_of_digit(mut n: usize) -> usize {
    let mut ans = 0;
    while n != 0 {
        n /= 10;
        ans += 1;
    }
    ans
}

fn solve(n: usize, v: &Vec<u64>, mut hs_v: &mut HashSet<Vec<u64>>) -> usize {
    let mut data = v.clone();
    // `HashSet` の重複回避
    // data.len() <= 9 であり十分高速
    data.sort();
    if hs_v.contains(&data) {
        return 0;
    }

    hs_v.insert(data.clone());
    let mut ans = 0;
    let mut num_p = 0;
    let mut hs_num = HashSet::new();
    heap_recursive(&mut data, |p| {
        let mut current = 0;
        for i in 0..v.len() {
            current *= 10;
            current += p[i];
        }
        // 重複する組み合わせを回避
        if !hs_num.contains(&current) {
            hs_num.insert(current);
            if current as usize <= n {
                // println!("{}", current);
                ans += 1;
            }
            num_p += 1;
        }
    });
    // 桁数基準で再帰止めてやらないと呼び出し回数が三度も多くなる (それが何倍にもなる)
    if ans != 0 && ans == num_p && num_of_digit(n) > data.len() {
        let mut v3 = v.clone();
        v3.push(3);
        let mut v5 = v.clone();
        v5.push(5);
        let mut v7 = v.clone();
        v7.push(7);
        ans + solve(n, &v3, &mut hs_v) + solve(n, &v5, &mut hs_v) + solve(n, &v7, &mut hs_v)
    } else {
        ans
    }
}

fn main() {
    input! {
        n: usize,
    }

    let base = vec![3, 5, 7];
    println!("{}", solve(n, &base, &mut HashSet::new()));
}
