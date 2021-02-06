// :fu: :fu: :fu: :fu: :fu: 数問 (幾何)
// ABC-C 問題史上最難易度では？

// use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }

    let mut is_edge = vec![vec![false; w]; h];

    for i in 1..h - 1 {
        for j in 1..w - 1 {
        }
    }


    // for i in 0..h {
    //     for j in 0..w {
    //         is_edge[i][j] = if shw[i][j] == '#' {
    //             true
    //         } else {
    //             false
    //         };
    //     }
    // }
    // // println!("{:?}", is_edge);

    // // 左から右
    // for i in 1..h {
    //     for j in 1..w {
    //         if !is_edge[i][j - 1] && shw[i][j] =='#' {
    //             is_edge[i][j] = true;
    //         }
    //     }
    // }
    // // 右から左
    // for i in 1..h {
    //     for j in 0..w - 1 {
    //         if !is_edge[i][j + 1] && shw[i][j] == '#' {
    //             is_edge[i][j] = true;
    //         }
    //     }
    // }
    // // println!("{:?}", is_edge);

    // // 上から下
    // for j in 1..w {
    //     for i in 1..h {
    //         if !is_edge[i - 1][j] && shw[i][j] == '#' {
    //             is_edge[i][j] = true;
    //         }
    //     }
    // }
    // // 下から上
    // for j in 1..w {
    //     for i in 0..h - 1 {
    //         if !is_edge[i + 1][j] && shw[i][j] == '#' {
    //             is_edge[i][j] = true;
    //         }
    //     }
    // }
    // println!("{:?}", is_edge);

    // for i in 1..h - 1 {
    //     for j in 1..w - 1 {
    //         if !is_edge[i][j] {
    //             continue;
    //         }

    //         if is_edge[i][j - 1] && is_edge[i][j + 1] {
    //             if (!is_edge[i + 1][j] && !is_edge[i - 1][j]) || !(is_edge[i - 1][j - 1] && is_edge[i - 1][j + 1]) {
    //                 is_edge[i][j] = false;
    //             }
    //         }
    //     }
    // }

    // for j in 1..w - 1 {
    //     for i in 1..h - 1 {
    //         if !is_edge[i][j] {
    //             continue;
    //         }

    //         if is_edge[i - 1][j] && is_edge[i + 1][j] {
    //             if (!is_edge[i][j + 1] && !is_edge[i][j - 1]) || !(is_edge[i - 1][j - 1] && is_edge[i + 1][j - 1]) {
    //                 is_edge[i][j] = false;
    //             }
    //         }
    //     }
    // }
    println!("{:?}", is_edge);


    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if is_edge[i][j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
