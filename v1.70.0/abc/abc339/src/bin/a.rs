use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = vec![];
    for c in s {
        if c == '.' {
            ans.clear();
        } else {
            ans.push(c);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
