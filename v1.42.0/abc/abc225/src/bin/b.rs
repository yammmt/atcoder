use proconio::input;

fn main() {
    input! {
        n: usize,
        abn1: [(usize, usize); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for ab in &abn1 {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }

    println!(
        "{}",
        if edges.iter().any(|a| a.len() == n - 1) {
            "Yes"
        } else {
            "No"
        }
    );
}
