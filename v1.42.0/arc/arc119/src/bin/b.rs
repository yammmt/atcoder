// :fu: :fu: 21-05 誤読 ARC っぽい 嫌い

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    let s0_num = s.iter().filter(|&c| *c == '0').count();
    let t0_num = t.iter().filter(|&c| *c == '0').count();
    if s0_num != t0_num {
        println!("-1");
        return;
    } else if s == t {
        println!("0");
        return;
    }

    let mut s_idx = vec![];
    let mut t_idx = vec![];
    for i in 0..n {
        if s[i] == '0' {
            s_idx.push(i);
        }
        if t[i] == '0' {
            t_idx.push(i);
        }
    }
    // println!("{:?}", s_idx);
    // println!("{:?}", t_idx);

    let mut ans = 0;
    for i in 0..s_idx.len() {
        if s_idx[i] != t_idx[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
