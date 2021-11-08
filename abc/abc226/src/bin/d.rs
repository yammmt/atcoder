// WA: GCD という概念そのものを忘れていた

use proconio::input;
use std::collections::HashSet;

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        xyn: [(isize, isize); n],
    }

    // すべての点を始点として残り全点までの距離を求める
    // 求めた距離から最大公約数を出して答えの集合に挿入し, 最後に要素数を数える
    // 約数に負数の概念が出てくるのが厄介だが公式解説はスルー (数弱)
    let mut ans = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }

            let cur_x = xyn[j].0 - xyn[i].0;
            let cur_y = xyn[j].1 - xyn[i].1;
            // 制約より (cur_x, cur,y) != (0, 0)
            if cur_x == 0 {
                ans.insert((0, cur_y / cur_y.abs()));
                continue;
            } else if cur_y == 0 {
                ans.insert((cur_x / cur_x.abs(), 0));
                continue;
            }

            let g = gcd(cur_x.abs(), cur_y.abs());
            ans.insert((cur_x / g, cur_y / g));
        }
    }
    // println!("{:?}", ans);

    println!("{}", ans.len());
}
