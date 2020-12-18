// :fu:

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [[i8; w]; h],
    }

    let mut ans = vec![];
    let mut cans = vec![];
    let mut h_idx = 0;
    let mut w_idx = 0;
    let mut w_increasing = true;
    while h_idx < h {
        match ahw[h_idx][w_idx] % 2 {
            0 => {
                if !cans.is_empty() {
                    cans.push((h_idx, w_idx));
                }
            },
            _ => {
                cans.push((h_idx, w_idx));
                if cans.len() > 1 {
                    for a in 1..cans.len() {
                        ans.push(format!("{} {} {} {}", cans[a - 1].0 + 1, cans[a - 1].1 + 1, cans[a].0 + 1, cans[a].1 + 1));
                    }
                    cans.clear();
                }
            }
        }

        match w_increasing {
            true => {
                if w_idx == w - 1 {
                    w_increasing = false;
                    h_idx += 1;
                } else {
                    w_idx += 1;
                }
            },
            false => {
                if w_idx == 0 {
                    w_increasing = true;
                    h_idx += 1;
                } else {
                    w_idx -= 1;
                }
            }
        }
    }

    println!("{}", ans.len());
    for a in &ans {
        println!("{}", a);
    }
}
