// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let vc: Vec<char> = s.chars().collect();
    let mut vx = vec![0; 26];
    let mut vy = vec![0; 26];
    for c in &vc {
        vy[(*c as u8 - b'a') as usize] += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        vx[(vc[i] as u8 - b'a') as usize] += 1;
        vy[(vc[i] as u8 - b'a') as usize] -= 1;

        let mut current = 0;
        for j in 0..26 {
            if vx[j] * vy[j] > 0 {
                current += 1;
            }
        }
        ans = ans.max(current);
    }
    println!("{}", ans);
}
