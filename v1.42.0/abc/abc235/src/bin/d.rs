// 13min

use proconio::input;
use std::collections::VecDeque;

const DUMMY: usize = std::usize::MAX / 3;

fn main() {
    input! {
        a: usize,
        n: usize,
    }

    // n < 10^6 かつ桁数は減らせない
    let mut counter = vec![DUMMY; 1_000_000];
    let mut vdq = VecDeque::new();
    vdq.push_back(1);
    counter[1] = 0;
    while let Some(cur) = vdq.pop_front() {
        let ax = cur * a;
        if ax < counter.len() && counter[ax] == DUMMY {
            if ax == n {
                println!("{}", counter[cur] + 1);
                return;
            }

            vdq.push_back(ax);
            counter[ax] = counter[cur] + 1;
        }
        if cur >= 10 && cur % 10 != 0 {
            let mut keta = 1;
            let mut _cur = cur;
            while _cur > 0 {
                keta *= 10;
                _cur /= 10;
            }
            keta /= 10;

            let nx = cur / 10 + (cur % 10) * keta;
            if nx < counter.len() && counter[nx] == DUMMY {
                if nx == n {
                    println!("{}", counter[cur] + 1);
                    return;
                }

                vdq.push_back(nx);
                counter[nx] = counter[cur] + 1;
            }
        }
    }

    println!("-1");
}
