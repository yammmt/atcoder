use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = Vec::with_capacity(n);
    for c in s {
        let mut pos = c as u8 - b'A';
        pos = (pos + n as u8) % 26;
        ans.push((b'A' + pos) as char);
    }

    println!("{}", ans.iter().collect::<String>());
}
