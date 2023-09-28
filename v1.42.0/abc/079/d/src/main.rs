// -*- coding:utf-8-unix -*-

// 27min.
// Debug ビルドだと TLE

// 想定解はワーシャルフロイド

use permutohedron::heap_recursive;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [[i32; 10]; 10],
        a: [[i32; w]; h],
    }

    let mut vd_to_1 = vec![std::i32::MAX; 10];
    vd_to_1[1] = 0;
    let mut data = (0..10).collect::<Vec<usize>>();
    // 10! = 3_628_800 < 4 * 10^6 であり間に合う
    heap_recursive(&mut data, |p| {
        let start = p[0];
        if start != 1 {
            // println!("{:?}", p);
            let mut cost = 0;
            for i in 0..p.len() - 1 {
                cost += c[p[i]][p[i + 1]];
                // println!("cost: {}", cost);
                if cost >= vd_to_1[start] {
                    break;
                }

                if p[i + 1] == 1 {
                    vd_to_1[start] = vd_to_1[start].min(cost);
                    break;
                }
            }
        }
    });
    // println!("{:?}", vd_to_1);

    let mut num_of_num = vec![0; 10];
    for hh in 0..h {
        for ww in 0..w {
            if a[hh][ww] != -1 {
                num_of_num[a[hh][ww] as usize] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..10 {
        ans += num_of_num[i] * vd_to_1[i];
    }
    println!("{}", ans);
}
