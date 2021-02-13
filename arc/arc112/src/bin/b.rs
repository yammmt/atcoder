// :fu: :fu: :fu: 数問 この点数帯の癖に手が出ない

// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        b: i64,
        c: i64,
    }

    // let mut ans = if b < 0 {
    //     1
    // } else {
    //     0
    // };
    let mut ans = 0;
    // let bb = b.abs();
    // let cc = if b < 0 {
    //     c - 1
    // } else {
    //     c
    // };

    if b > 0 {
        if c > 2 {
            println!("{}", c + 1);
            return;
        }

        ans += 3;
        // 0 が出るまで + 2 ずつ
        // 0 が出てから + 1 ずつ
    }

    // if b > 0 && b - c / 2 < 0 {
    //     let range1 = (b - c / 2, b);
    //     let range2 = (, -b);
    //     let range3 = (,);
    //     ans = range1.1 - range1.0 + range2.1 - range2.0 + range3.1 - range3.0 + 3;
    // }

    // let from_minus_minus_min = bb - (cc / 2);
    // let from_minus_minus_max = bb;
    // let from_minus_rev_min = 1 - bb;
    // let from_minus_rev_max = (cc - 2) / 2 - b;

    // let from_rev_minus_min = -bb - (cc - 1) / 2;
    // let from_rev_minus_max = -bb;
    // let from_rev_rev_min = bb;
    // let from_rev_rev_max = if (cc - 1) % 2 == 0 {
    //     (from_rev_minus_max - 1) * (-1)
    // } else {
    //     from_rev_minus_min * (-1)
    // };

    // 初手 -1
    // let mut ans = c / 2 + 1;
    // ans += if c % 2 == 0 {
    //     c / 2 - 1
    // } else {
    //     c / 2
    // };
    // // println!("{}", ans);

    // 初手 * (-1)
    // if b != 0 {
    //     ans += 1; // -b
    //     let cc = c - 1;
    //     if cc > 1 {
    //         ans += cc / 2;
    //         ans += if cc % 2 == 0 {
    //             cc / 2
    //         } else {
    //             (cc / 2 - 1).abs()
    //         };
    //     }
    // }

    println!("{}", ans);
}
