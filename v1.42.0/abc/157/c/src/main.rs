// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u16,
        m: u16,
    }

    let mut v = vec![0; n.into()];
    let mut appeared_degree = vec!();
    for _i in 0..m {
        input! {
            s: usize,
            c: u16,
        }

        if n != 1 && s == 1 && c ==0 {
            println!("-1");
            return;
        } else if appeared_degree.contains(&s) {
            if c == v[(s - 1) as usize] {
                continue;
            }
            else {
                // println!("s: {}, c: {}", s, c);
                // println!("{:?}", v);
                println!("-1");
                return;
            }
        }

        appeared_degree.push(s);
        v[(s - 1) as usize] = c
    }

    v.reverse();
    let mut ans: u16 = 0;
    for i in 0..v.len() {
        if n != 1 && (i == v.len() - 1) && v[i as usize] == 0 {
            ans += 1 * 10_u16.pow(i as u32);
        } else {
            ans += v[i as usize] * 10_u16.pow(i as u32);
        }
    }
    println!("{}", ans);
}
