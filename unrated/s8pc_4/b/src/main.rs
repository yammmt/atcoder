// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }

    let mut ans = std::i64::MAX;
    for bit_row in 0..2u32.pow(n as u32) {
        // selected の建物で条件を満たす
        let mut selected = vec![];
        for i in 0..n {
            if bit_row & (1 << i) > 0 {
                selected.push(i);
            }
        }
        if selected.len() != k {
            continue;
        }
        // println!("{:?}", selected);

        let mut current = 0;
        let mut required = 0;
        let mut selected_idx = 0;
        for i in 0..n {
            if i == selected[selected_idx] {
                // println!("required: {}", required);
                if required > a[i] {
                    current += required - a[i];
                    required = required + 1;
                } else {
                    required = a[i] + 1;
                }
                selected_idx += 1;
                if selected_idx == k {
                    break;
                }
            } else {
                required = required.max(a[i] + 1);
            }
        }
        // println!("{}", current);
        ans = ans.min(current);
    }
    println!("{}", ans);
}
