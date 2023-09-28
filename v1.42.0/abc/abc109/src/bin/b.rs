use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        wn: [Chars; n],
    }

    let mut said = HashSet::new();
    for w in &wn {
        said.insert(w.iter().collect::<String>());
    }
    if said.len() != n {
        println!("No");
        return;
    }

    for i in 1..n {
        if wn[i - 1][wn[i - 1].len() - 1] != wn[i][0] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
