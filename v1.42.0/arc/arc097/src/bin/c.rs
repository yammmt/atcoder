// :fu: :fu: 21-12 解けなくなった

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let mut vs = vec![];
    for i in 0..s.len() {
        let mut cur = vec![];
        for j in i..(i + k + 1).min(s.len()) {
            cur.push(s[j]);
            vs.push(cur.iter().collect::<String>());
        }
    }

    vs.sort_unstable();
    vs.dedup();
    println!("{}", vs[k - 1]);
}
