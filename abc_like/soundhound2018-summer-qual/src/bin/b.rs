use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        w: usize,
    }

    let mut ans = vec![];
    let mut idx = 0;
    while idx < s.len() {
        ans.push(s[idx]);
        idx += w;
    }

    println!("{}", ans.iter().collect::<String>());
}
