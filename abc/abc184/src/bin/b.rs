use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut x: i64,
        s: Chars,
    }

    for c in &s {
        if *c == 'o' {
            x += 1;
        } else {
            x = (x - 1).max(0);
        }
    }
    println!("{}", x);
}
