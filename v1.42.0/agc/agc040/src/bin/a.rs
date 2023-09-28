// :fu: :fu:

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut v = vec![0; s.len() + 1];
    let mut streak = 0;
    for i in 0..s.len() {
        if s[i] == '<' {
            streak += 1;
            v[i + 1] = streak;
        } else {
            streak = 0;
        }
    }

    let mut streak = 0;
    for i in (0..s.len()).rev() {
        if s[i] == '>' {
            streak += 1;
            v[i] = v[i].max(streak);
        } else {
            streak = 0;
        }
    }
    // println!("{:?}", v);

    println!("{}", v.iter().sum::<u64>());
}
