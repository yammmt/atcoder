// A としては重い

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut v = vec![
        vec![s[0], s[1], s[2]].iter().collect::<String>(),
        vec![s[0], s[2], s[1]].iter().collect::<String>(),
        vec![s[1], s[0], s[2]].iter().collect::<String>(),
        vec![s[1], s[2], s[0]].iter().collect::<String>(),
        vec![s[2], s[0], s[1]].iter().collect::<String>(),
        vec![s[2], s[1], s[0]].iter().collect::<String>(),
    ];
    v.sort_unstable();
    v.dedup();
    println!("{}", v.len());
}
