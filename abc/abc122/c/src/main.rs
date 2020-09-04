// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
    }

    let vc: Vec<char> = s.chars().collect();
    let mut score = Vec::with_capacity(n);
    score.push(0);
    let mut sum = 0;
    for i in 1..n {
        if vc[i - 1] == 'A' && vc[i] == 'C' {
            sum += 1;
        }
        score.push(sum);
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", score[r - 1] - score[l - 1]);
    }
}
