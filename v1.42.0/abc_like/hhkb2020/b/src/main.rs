// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut s = vec![];
    for _ in 0..h {
        input! {
            st: String,
        }
        let vc = st.chars().collect::<Vec<char>>();
        s.push(vc);
    }
    // println!("{:?}", s);

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if j > 0 && s[i][j] == '.' && s[i][j - 1] == '.' {
                ans += 1;
            }
        }
    }
    for j in 0..w {
        for i in 0..h {
            if i > 0 && s[i - 1][j] == '.' && s[i][j] == '.' {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
