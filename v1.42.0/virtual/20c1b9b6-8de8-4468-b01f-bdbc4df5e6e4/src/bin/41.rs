use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut counted_white_num = 0;
    for (i, c) in s.iter().enumerate() {
        if c == &'W' {
            ans += i - counted_white_num;
            counted_white_num += 1;
        }
    }

    println!("{}", ans);
}
