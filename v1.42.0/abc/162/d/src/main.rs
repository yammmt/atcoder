// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let vc: Vec<char> = s.chars().collect();
    let mut vidx = vec![vec![]; 3];
    for i in 0..n {
        match vc[i] {
            'R' => vidx[0].push(i as u32),
            'G' => vidx[1].push(i as u32),
            'B' => vidx[2].push(i as u32),
            _ => unreachable!(),
        }
    }

    let mut ans = vidx[0].len() * vidx[1].len() * vidx[2].len();
    for i in 0..n {
        for j in i + 1..n {
            if vc[i] == vc[j] {
                continue;
            }

            let k = 2 * j - i;
            if k >= n || vc[k] == vc[i] || vc[k] == vc[j] {
                continue;
            }

            ans -= 1;
        }
    }

    println!("{}", ans);
}
