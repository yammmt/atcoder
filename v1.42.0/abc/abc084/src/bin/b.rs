use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: usize,
        _b: usize,
        s: Chars,
    }

    for (i, c) in s.iter().enumerate() {
        if i == a {
            if *c != '-' {
                println!("No");
                return;
            }
        } else if *c == '-' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
