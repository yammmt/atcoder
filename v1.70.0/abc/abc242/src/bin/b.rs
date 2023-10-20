use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }

    s.sort_unstable();

    println!("{}", s.iter().collect::<String>());
}
