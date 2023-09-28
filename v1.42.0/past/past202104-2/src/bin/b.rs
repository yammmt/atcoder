use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    for (i, c) in s.iter().enumerate() {
        if i % 4 == 0 {
            ans += 1;
        }

        if i % 4 == 1 && *c == 'o' {
            println!("{}", ans);
            return;
        }
    }

    println!("none");
}
