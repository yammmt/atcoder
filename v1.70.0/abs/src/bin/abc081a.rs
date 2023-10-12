use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: Chars,
    }

    let ans = c.iter().filter(|&&a| a == '1').count();
    println!("{ans}");
}
