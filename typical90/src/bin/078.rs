use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }
    let mut edges = vec![vec![]; n];
    abm.iter().for_each(|ab| {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    });
    let edges = edges;

    let mut ans = 0;
    for i in 0..n {
        let mut small = 0;
        for e in &edges[i] {
            if *e < i {
                small += 1;
            }
        }
        if small == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
