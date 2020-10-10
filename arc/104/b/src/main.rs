// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let vc = s.chars().collect::<Vec<char>>();

    let mut ans = 0;
    for i in 0..n {
        let mut a_num = 0;
        let mut t_num = 0;
        let mut c_num = 0;
        let mut g_num = 0;
        for j in i..n {
            match vc[j] {
                'A' => a_num += 1,
                'T' => t_num += 1,
                'C' => c_num += 1,
                'G' => g_num += 1,
                _ => unreachable!(),
            }
            if a_num == t_num && c_num == g_num {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
