// -*- coding:utf-8-unix -*-

// サンプル通るが TLE/WA

use permutohedron::heap_recursive;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut w: [usize; n],
        lv: [(usize, usize); m],
    }

    // w <= 8 より sort は十分高速
    w.sort();
    // min(v[i]) < max(w[j]) が存在すれば不可
    let mut vmin = std::usize::MAX;
    for i in &lv {
        vmin = vmin.min(i.1);
    }
    if vmin < w[0] {
        println!("-1");
        return;
    }

    // 全通りの配置を試し, パターンごとの間隔の最小値が答え
    // パターンごとの間隔は区間ごとの間隔の最大値
    let mut ans = std::u64::MAX;
    heap_recursive(&mut w, |p| {
        let mut cans = 0u64;
        let mut lightest_v = lv[0].1 as u64;
        // 各区間ごとに先頭と末尾の差を求める
        for i in &lv {
            // println!("{:?}", i);
            let mut section_ans = 0;
            let mut total_weight = 0;
            for j in 0..p.len() {
                if total_weight + p[j] > i.1 {
                    section_ans += i.0 as u64;
                    total_weight = p[j];
                } else {
                    total_weight += p[j];
                }
            }
            cans = cans.max(section_ans);
            // 今空いている間隔は同時に橋を渡る重さが `lightest_v` 以下
            if lightest_v > i.1 as u64 && cans != section_ans {
                // println!("+= {}", i.1);
                cans += i.0 as u64;
            }
            lightest_v = lightest_v.min(i.1 as u64);
            // println!("{:?}", lightest_v);
        }
        // panic!();
        // println!("p: {:?}", p);
        // println!("cans: {:?}", cans);
        ans = ans.min(cans);
    });
    println!("{}", ans);
}
