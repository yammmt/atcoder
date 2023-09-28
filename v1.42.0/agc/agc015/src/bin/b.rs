// 20min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = s.len() * (s.len() - 1);
    // println!("{}", ans);
    for i in 0..s.len() {
        if s[i] == 'U' {
            // 下に行く際に一度上がる
            ans += i;
        } else if i != s.len() - 1 {
            // 上に行く際に一度下がる
            ans += s.len() - i - 1;
        }
        // println!("{}", ans);
    }
    println!("{}", ans);
}
