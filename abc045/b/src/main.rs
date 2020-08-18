// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        sa: String,
        sb: String,
        sc: String,
    }

    let va: Vec<char> = sa.chars().collect();
    let vb: Vec<char> = sb.chars().collect();
    let vc: Vec<char> = sc.chars().collect();
    let v: Vec<Vec<char>> = vec!(va, vb, vc);
    let mut turn = 0;
    let mut card_idx = vec!(0, 0, 0);
    loop {
        if card_idx[turn] == v[turn].len() {
            match turn {
                0 => println!("A"),
                1 => println!("B"),
                2 => println!("C"),
                _ => unreachable!(),
            }
            return;
        }

        card_idx[turn] += 1;
        turn = match v[turn][card_idx[turn] - 1] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => unreachable!(),
        };
    }
}
