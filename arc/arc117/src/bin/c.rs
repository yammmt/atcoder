// TLE もうしばらくしてから

use proconio::input;
use proconio::marker::Chars;

fn block_color(a: char, b: char) -> char {
    if a == b {
        a
    } else {
        if a == 'B' {
            if b == 'R' {
                'W'
            } else {
                'R'
            }
        } else if a == 'W' {
            if b == 'B' {
                'R'
            } else {
                'B'
            }
        } else if a == 'R' {
            if b == 'B' {
                'W'
            } else {
                'B'
            }
        } else {
            unreachable!()
        }
    }
}

fn main() {
    input! {
        _n: usize,
        cn: Chars,
    }

    let mut before = vec![];
    for c in &cn {
        before.push(*c);
    }

    while before.len() > 1 {
        let mut after = vec![];
        for i in 0..before.len() - 1 {
            after.push(block_color(before[i], before[i + 1]));
        }
        before = after;
    }

    println!("{}", before[0]);
}
