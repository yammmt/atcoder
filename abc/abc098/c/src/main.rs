// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let vc: Vec<char> = s.chars().collect();
    let mut east_dir = (
        vc.iter().filter(|&c| *c == 'W').count() as u32,
        vc.iter().filter(|&c| *c == 'E').count() as u32,
    );
    let mut west_dir: (u32, u32) = (0, 0);
    let mut ans = n as u32 + 1;
    for i in 0..n {
        let mut humans = west_dir.0 + east_dir.1;
        if vc[i] == 'E' && east_dir.1 > 0 {
            humans -= 1;
        }
        if humans < ans {
            ans = humans;
        }

        if vc[i] == 'W' {
            west_dir.0 += 1;
            east_dir.0 -= 1;
        } else {
            west_dir.1 += 1;
            east_dir.1 -= 1;
        }
    }
    println!("{}", ans);
}
