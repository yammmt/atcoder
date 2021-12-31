use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let mut vs = vec![];
    for perm in (0..s.len()).permutations(s.len()) {
        let mut vc = vec![];
        (0..s.len()).for_each(|i| vc.push(s[perm[i]]));
        vs.push(vc.iter().collect::<String>());
    }

    vs.sort_unstable();
    vs.dedup();
    println!("{}", vs[k - 1]);
}
