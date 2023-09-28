use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        b: Chars,
    }
    let mut ans = vec![];
    for i in 0..n {
        ans.push(s[i]);
        ans.push(b[i]);
    }
    println!("{}", ans.iter().collect::<String>());
}
