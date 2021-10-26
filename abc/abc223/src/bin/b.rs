use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut candidates = vec![];
    for i in 0..s.len() {
        let mut cur = vec![];
        for j in 0..s.len() {
            cur.push(s[(i + j) % s.len()]);
        }
        candidates.push(cur.iter().collect::<String>());
    }
    candidates.sort_unstable();

    println!("{}", candidates.first().unwrap());
    println!("{}", candidates.last().unwrap());
}
