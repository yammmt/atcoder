// L にしてはやるだけでは？ と思ったが毎回の操作後に値を出すわけで

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut hn: [i64; n],
    }

    // (左 - 右) の値を全点分見て保持する
    // 一括加算に対しては見るべき間隔を調整、一点追加時には HashMap の二要素を入れ直しとすれば
    // O(N) で計算ができる

    let mut odd_left_minus_right = HashMap::new();
    let mut even_left_minus_right = HashMap::new();
    for i in 0..n - 1 {
        let cnt = if i % 2 == 0 {
            odd_left_minus_right.entry(hn[i] - hn[i + 1]).or_insert(0)
        } else {
            even_left_minus_right.entry(hn[i] - hn[i + 1]).or_insert(0)
        };
        *cnt += 1;
    }
    // println!("{:?}", odd_left_minus_right);
    // println!("{:?}", even_left_minus_right);

    let mut odd_added = 0;
    for _ in 0..q {
        input! {
            q0: usize,
        }
        match q0 {
            1 => {
                input! {
                    v: i64,
                }
                odd_added += v;
            }
            2 => {
                input! {
                    v: i64,
                }
                odd_added -= v;
            }
            3 => {
                input! {
                    mut u: usize,
                    v: i64,
                }
                let u = u - 1;
                // TODO: 汚い
                let new_h = hn[u] + v;
                if u % 2 == 0 {
                    // 自身とその左の差分
                    if u > 0 {
                        let cnt = even_left_minus_right.entry(hn[u - 1] - hn[u]).or_insert(0);
                        *cnt -= 1;
                        let cnt = even_left_minus_right.entry(hn[u - 1] - new_h).or_insert(0);
                        *cnt += 1;
                    }
                    // 自身とその右の差分
                    if u < n - 1 {
                        let cnt = odd_left_minus_right.entry(hn[u] - hn[u + 1]).or_insert(0);
                        *cnt -= 1;
                        let cnt = odd_left_minus_right.entry(new_h - hn[u + 1]).or_insert(0);
                        *cnt += 1;
                    }
                } else {
                    // 自身とその左の差分
                    if u > 0 {
                        let cnt = odd_left_minus_right.entry(hn[u - 1] - hn[u]).or_insert(0);
                        *cnt -= 1;
                        let cnt = odd_left_minus_right.entry(hn[u - 1] - new_h).or_insert(0);
                        *cnt += 1;
                    }
                    // 自身とその右の差分
                    if u < n - 1 {
                        let cnt = even_left_minus_right.entry(hn[u] - hn[u + 1]).or_insert(0);
                        *cnt -= 1;
                        let cnt = even_left_minus_right.entry(new_h - hn[u + 1]).or_insert(0);
                        *cnt += 1;
                    }
                }
                hn[u] = new_h;
                // println!("{:?}", left_minus_right);
            }
            _ => unreachable!(),
        }
        // println!("  {:?}", odd_left_minus_right);
        // println!("  {:?}", even_left_minus_right);
        println!(
            "{}",
            odd_left_minus_right.get(&-odd_added).unwrap_or(&0)
                + even_left_minus_right.get(&odd_added).unwrap_or(&0)
        );
    }
}
