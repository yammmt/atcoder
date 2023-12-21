// multiset なし言語だと大変

use proconio::fastout;
use proconio::input;
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut btm = BTreeMap::new();
    for _ in 0..q {
        input! {
            i: usize,
        }
        match i {
            1 => {
                input! {
                    x: usize,
                }
                if let Some(cur) = btm.get(&x) {
                    btm.insert(x, cur + 1);
                } else {
                    btm.insert(x, 1);
                }
            },
            2 => {
                input! {
                    x: usize,
                    c: usize,
                }
                if let Some(&cur) = btm.get(&x) {
                    if cur > c {
                        btm.insert(x, cur - c);
                    } else {
                        btm.remove(&x);
                    }
                }
            }
            3 => {
                let vmin = btm.pop_first().unwrap();
                if btm.is_empty() {
                    println!("0");
                    btm.insert(vmin.0, vmin.1);
                } else {
                    let vmax = btm.pop_last().unwrap();
                    println!("{}", vmax.0 - vmin.0);
                    btm.insert(vmin.0, vmin.1);
                    btm.insert(vmax.0, vmax.1);
                }
            }
            _ => unreachable!(),
        }
    }
}
