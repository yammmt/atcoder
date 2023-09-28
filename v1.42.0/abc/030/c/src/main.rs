// -*- coding:utf-8-unix -*-

// 8min

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: u64,
        y: u64,
        an: [u64; n],
        bm: [u64; m],
    }

    let mut ans = 0;
    let mut ctime = 0;
    let mut aidx = 0;
    let mut bidx = 0;
    let mut a_turn = true;
    loop {
        if a_turn {
            let mut ai = std::usize::MAX;
            for i in aidx..n {
                if an[i] >= ctime {
                    ai = i;
                    break;
                }
            }
            if ai == std::usize::MAX {
                break;
            }

            ctime = an[ai] + x;
            aidx = ai + 1;
            a_turn = false;
        } else {
            let mut bi = std::usize::MAX;
            for i in bidx..m {
                if bm[i] >= ctime {
                    bi = i;
                    break;
                }
            }
            if bi == std::usize::MAX {
                break;
            }

            ctime = bm[bi] + y;
            bidx = bi + 1;
            ans += 1;
            a_turn = true;
        }
    }
    println!("{}", ans);
}
