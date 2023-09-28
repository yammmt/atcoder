use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    for i in 0..s.len() {
        if i % 2 == 0 {
            if s[i] == 'L' {
                println!("No");
                return;
            }
        } else if s[i] == 'R' {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
