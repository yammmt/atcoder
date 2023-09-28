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
    } else {
        let mut zeronum = t.iter().filter(|&c| *c == '0').count() as u64;
        if t[t.len() - 1] != '0' {
            zeronum += 1;
        }
        println!("{}", 10u64.pow(10) - zeronum + 1);
    }
}
