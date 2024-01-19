use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        hn: [usize; n],
        abm: [(Usize1, Usize1); m],
    }

    let mut edges = vec![vec![]; n];
    for (a, b) in abm {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut ans = 0;
    for i in 0..n {
        let mut is_good = true;
        for v in &edges[i] {
            if hn[*v] >= hn[i] {
                is_good = false;
            }
        }
        if is_good {
            ans += 1;
        }
    }

    println!("{ans}");
}
