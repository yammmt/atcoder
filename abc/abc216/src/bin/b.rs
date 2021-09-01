use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut stn: [(Chars, Chars); n],
    }

    let mut hs = HashSet::new();
    for st in stn.iter_mut() {
        let s = &st.0;
        let t = &st.1;
        let mut cur = vec![];
        s.iter().for_each(|c| {
            cur.push(c);
        });
        cur.push(&' ');
        t.iter().for_each(|c| {
            cur.push(c);
        });
        if hs.contains(&cur) {
            println!("Yes");
            return;
        }
        hs.insert(cur);
    }

    println!("No");
}
