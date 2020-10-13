// -*- coding:utf-8-unix -*-

// 37min.

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let vs = s.chars().collect::<Vec<char>>();

    let mut ans = vec![0; vs.len()];
    // 左端から R に進んでいって L に辿り着くまでの移動回数
    let mut first_r = std::usize::MAX;
    for i in 0..vs.len() {
        if vs[i] == 'R' && first_r == std::usize::MAX {
            first_r = i;
            continue;
        }

        if vs[i - 1] == 'R' && vs[i] == 'L' {
            let section_len = i - first_r;
            ans[i] += section_len / 2;
            ans[i - 1] += section_len / 2 + section_len % 2;
            first_r = std::usize::MAX; // reset
        }
    }
    // 右端から L に進んでいって R に辿り着くまでの移動回数
    let mut first_l = std::usize::MAX;
    for i in (0..vs.len()).rev() {
        if vs[i] == 'L' && first_l == std::usize::MAX {
            first_l = i;
            continue;
        }

        if vs[i] == 'R' && vs[i + 1] == 'L' {
            let section_len = first_l - i;
            ans[i] += section_len / 2;
            ans[i + 1] += section_len / 2 + section_len % 2;
            first_l = std::usize::MAX; // reset
        }
    }

    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i == ans.len() - 1 {
            println!("");
        } else {
            print!(" ");
        }
    }
}
