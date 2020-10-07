// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let vn = s.chars().map(|a| a.to_digit(10).unwrap() as u64).collect::<Vec<_>>();

    let n = (s.len() - 1) as usize;
    if n == 0 {
        println!("{}", vn[0]);
        return;
    }

    let mut ans = 0;
    for bit_row in 0..2u32.pow(n as u32) {
        let mut selected = vec![];
        for i in 0..n {
            if bit_row & (1 << i) > 0 {
                selected.push(i);
            }
        }

        // println!("selected: {:?}", selected);
        let mut current_sum = 0;
        let mut current_num = 0;
        for i in 0..s.len() {
            if i == 0 {
                current_num += vn[0];
                continue;
            }

            if selected.contains(&(i - 1)) {
                current_sum += current_num;
                current_num = vn[i];
            } else {
                current_num *= 10;
                current_num += vn[i];
            }

            if i == s.len() - 1 {
                current_sum += current_num;
            }
        }
        // println!("{}", current_sum);
        ans += current_sum;
    }
    println!("{}", ans);
}
