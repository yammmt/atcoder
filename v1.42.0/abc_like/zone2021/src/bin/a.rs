use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    for i in 0..11 {
        if s[i] == 'Z' && i + 3 < 12 && s[i + 1] == 'O'
            && s[i + 2] == 'N' && s[i + 3] == 'e'
        {
            ans += 1;
        }
    }
    println!("{}", ans);
}
