use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
        k: usize,
    }

    let written = s[k - 1];
    for c in &mut s {
        if *c != written {
            *c = '*';
        }
    }
    println!("{}", s.iter().collect::<String>());
}
