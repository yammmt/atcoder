// -*- coding:utf-8-unix -*-

// https://www.slideshare.net/chokudai/abc024

use proconio::input;

fn main() {
    input! {
        _n: usize,
        d: usize,
        k: usize,
        lr: [(usize, usize); d],
        st: [(usize, usize); k],
    }

    for sg in &st {
        let mut current = sg.0;
        let mut ans = 0;
        for p in &lr {
            ans += 1;
            if !(p.0 <= current && current <= p.1) {
                continue;
            }

            if p.0 <= sg.1 && sg.1 <= p.1 {
                println!("{}", ans);
                break;
            }
            current = if sg.0 < sg.1 {
                p.1
            } else {
                p.0
            };
        }
    }
}
