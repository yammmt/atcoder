// :fu:

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: i64,
        t: Chars,
    }

    for i in 1..t.len() {
        if t[i] == '1' {
            // 111
            if i >= 2 && t[i - 1] == '1' && t[i - 2] == '1' {
                println!("0");
                return;
            }
        } else {
            // t[i] == '0'
            // 00 or 010
            if t[i - 1] == '0' || (i >= 2 && t[i - 1] == '1' && t[i - 2] == '0') {
                println!("0");
                return;
            }
        }
    }

    if t.len() == 1 {
        // pass
        if t[0] == '0' {
            println!("{}", 10u64.pow(10));
        } else {
            println!("{}", 10u64.pow(10) * 2);
        }
    } else if (t.len() == 2 && t[0] == '1' && t[1] == '1') || (t.len() == 3 && t[2] == '0') {
        // pass
        // 11 or 110
        println!("{}", 10u64.pow(10));
    } else if t.len() % 3 == 0 && t[0] == '1' && t[1] == '1' && t[t.len() - 1] == '0' {
        // pass
        // 110110...
        println!("{}", 10u64.pow(10) - (t.len() / 3) as u64 + 1);
    } else {
        let mut loopnum = 0;
        for i in 1..t.len() {
            if t[i] == '1' && t[i - 1] == '0' {
                loopnum += 1;
            }
        }
        println!("{}", 10u64.pow(10) - loopnum as u64);
    }
}
