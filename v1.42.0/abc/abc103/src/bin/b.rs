use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    for _ in 0..s.len() {
        if s == t {
            println!("Yes");
            return;
        }

        let mut ss = s.clone();
        for i in 0..s.len() {
            ss[i] = s[(i + 1) % s.len()];
        }
        s = ss;
    }
    println!("No");
}
