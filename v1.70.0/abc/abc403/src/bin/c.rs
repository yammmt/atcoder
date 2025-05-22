use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    }

    let mut allowed_each = vec![HashSet::new(); n];
    let mut allowed_all = vec![false; n];

    for _ in 0..q {
        input! {
            qq: usize,
        }
        match qq {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                allowed_each[x].insert(y);
            }
            2 => {
                input! {
                    x: Usize1,
                }
                allowed_all[x] = true;
            }
            3 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                println!(
                    "{}",
                    if allowed_each[x].contains(&y) || allowed_all[x] {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}
