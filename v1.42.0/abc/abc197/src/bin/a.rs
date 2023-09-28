use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let ans = vec![s[1], s[2], s[0]];
    println!("{}", ans.iter().collect::<String>());
}
