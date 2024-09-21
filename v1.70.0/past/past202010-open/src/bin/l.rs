use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut hn: [isize; n],
    }

    // i+1 との差を持っておいてなんとかできるのかしら
    // 偶奇別に自身と右との差をもっておいて, 同じ高さ = 高さ変化量と言い換えて, でできそう
    // origin の都合で問題文とは偶奇が逆

    let mut cnt_e = HashMap::new();
    let mut cnt_o = HashMap::new();
    for i in 0..n - 1 {
        let diff = hn[i + 1] - hn[i];
        if i % 2 == 0 {
            let cnt = cnt_e.entry(diff).or_insert(0);
            *cnt += 1;
        } else {
            let diff = hn[i + 1] - hn[i];
            let cnt = cnt_o.entry(diff).or_insert(0);
            *cnt += 1;
        }
    }

    let mut diff_e = 0;
    let mut diff_o = 0;
    for _ in 0..q {
        input! {
            qq: usize,
        }
        match qq {
            1 => {
                input! {
                    v: isize,
                }
                diff_e += v;
            }
            2 => {
                input! {
                    v: isize,
                }
                diff_o += v;
            }
            3 => {
                input! {
                    u: Usize1,
                    v: isize,
                }
                if u > 0 {
                    // i-1, i
                    let diff_old = hn[u] - hn[u - 1];
                    let diff_new = diff_old + v;
                    if u % 2 == 0 {
                        let cnt = cnt_o.entry(diff_old).or_insert(0);
                        *cnt -= 1;
                        let cnt = cnt_o.entry(diff_new).or_insert(0);
                        *cnt += 1;
                    } else {
                        let cnt = cnt_e.entry(diff_old).or_insert(0);
                        *cnt -= 1;
                        let cnt = cnt_e.entry(diff_new).or_insert(0);
                        *cnt += 1;
                    }
                }
                if u < n - 1 {
                    // i, i+1
                    let diff_old = hn[u + 1] - hn[u];
                    let diff_new = diff_old - v;
                    if u % 2 == 0 {
                        let cnt = cnt_e.entry(diff_old).or_insert(0);
                        *cnt -= 1;
                        let cnt = cnt_e.entry(diff_new).or_insert(0);
                        *cnt += 1;
                    } else {
                        let cnt = cnt_o.entry(diff_old).or_insert(0);
                        *cnt -= 1;
                        let cnt = cnt_o.entry(diff_new).or_insert(0);
                        *cnt += 1;
                    }
                }
                hn[u] += v;
            }
            _ => unreachable!(),
        }

        let mut ans = 0;
        let o_minus_e = diff_o - diff_e;
        if let Some(ans_o) = cnt_o.get(&o_minus_e) {
            ans += ans_o;
        }
        if let Some(ans_e) = cnt_e.get(&-o_minus_e) {
            ans += ans_e;
        }
        println!("{ans}");
    }
}
