use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    for w in 1..s.len() {
        // TODO: 長さは削れる
        let mut vs = vec![vec![]; s.len()];
        for i in 0..s.len() {
            vs[i % w].push(s[i]);
        }

        for v in vs {
            if v == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
