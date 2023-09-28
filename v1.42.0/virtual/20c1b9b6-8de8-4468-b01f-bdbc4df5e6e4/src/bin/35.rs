use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    for offset in 0..s.len() {
        let mut pass = true;
        for i in 0..s.len() {
            if s[(offset + i) % s.len()] != t[i] {
                pass = false;
                break;
            }
        }
        if pass {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
