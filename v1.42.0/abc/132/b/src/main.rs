// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        pn: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n - 2 {
        let mut vi = vec![pn[i], pn[i + 1], pn[i + 2]];
        vi.sort();
        if vi[1] == pn[i + 1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
