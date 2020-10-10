// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    if s == t {
        println!("Yes");
        return;
    }

    let mut vs = s.chars().collect::<Vec<char>>();

    loop {
        let vs_str = vs.iter().collect::<String>();
        if vs_str == t {
            println!("Yes");
            return;
        }

        let mut vss = Vec::with_capacity(vs.len());
        for i in 0..vs.len() {
            if i == 0 {
                vss.push(vs[vs.len() - 1]);
                continue;
            }
            vss.push(vs[i - 1]);
        }
        vs = vss.clone();
        if vs.iter().collect::<String>() == s {
            println!("No");
            return;
        }
    }
}
