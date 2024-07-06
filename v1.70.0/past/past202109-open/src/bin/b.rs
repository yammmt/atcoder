use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
        bm: [usize; m],
    }

    let set_a: HashSet<usize> = HashSet::from_iter(an.into_iter());
    let mut ans = vec![];
    bm.iter()
        .filter(|b| set_a.contains(b))
        .for_each(|&b| ans.push(b));

    ans.sort_unstable();
    for (i, a) in ans.iter().enumerate() {
        print!("{a}");
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
