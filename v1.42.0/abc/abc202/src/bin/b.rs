use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }
    s.reverse();
    for i in 0..s.len() {
        match s[i] {
            '6' => s[i] = '9',
            '9' => s[i] = '6',
            _ => {},
        }
    }
    println!("{}", s.iter().collect::<String>());
}
