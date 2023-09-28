use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let atcoder: HashSet<char> = vec!['a', 't', 'c', 'o', 'd', 'e', 'r']
        .into_iter()
        .collect();
    for i in 0..s.len() {
        if s[i] != t[i]
            && !((s[i] == '@' && atcoder.contains(&t[i]))
                || (t[i] == '@' && atcoder.contains(&s[i])))
        {
            println!("You will lose");
            return;
        }
    }

    println!("You can win");
}
