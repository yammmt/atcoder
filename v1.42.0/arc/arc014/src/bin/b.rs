use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        wn: [Chars; n],
    }

    let mut said = HashSet::new();
    let mut last = None;
    for (i, w) in wn.iter().enumerate() {
        let s = w.iter().collect::<String>();
        if said.contains(&s) || (last.is_some() && Some(w[0]) != last) {
            println!("{}", if i % 2 == 0 { "LOSE" } else { "WIN" });
            return;
        }

        said.insert(s);
        last = Some(w[w.len() - 1]);
    }

    println!("DRAW");
}
