use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

fn nc3(n: usize) -> usize {
    if n < 3 {
        return 0;
    }

    n * (n - 1) * (n - 2) / 6
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(Usize1, Usize1); m],
    }

    let mut available = vec![n - 1; n];
    for (a, b) in abm {
        available[a] -= 1;
        available[b] -= 1;
    }

    let mut ans = Vec::with_capacity(n);
    (0..n).for_each(|i| ans.push(nc3(available[i])));

    println!("{}", ans.iter().join(" "));
}
