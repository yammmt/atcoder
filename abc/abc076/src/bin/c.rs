// 11.5min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    if s.len() < t.len() {
        println!("UNRESTORABLE");
        return;
    }

    let mut insert_pos = s.len() + 1;
    for i in 0..s.len() - t.len() + 1 {
        for j in 0..t.len() {
            if s[i + j] != t[j] && s[i + j] != '?' {
                break;
            }

            if j == t.len() - 1 {
                insert_pos = i;
            }
        }
    }

    if insert_pos == s.len() + 1 {
        println!("UNRESTORABLE");
    } else {
        for i in 0..s.len() {
            if s[i] == '?' {
                if i >= insert_pos && i < insert_pos + t.len() {
                    print!("{}", t[i - insert_pos]);
                } else {
                    print!("a");
                }
            } else {
                print!("{}", s[i]);
            }
        }
    }
    println!();
}
