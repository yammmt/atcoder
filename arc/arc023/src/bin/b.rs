use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        d: usize,
        arc: [[usize; c]; r],
    }

    let mut candidates = vec![(arc[0][0], 2)];
    for i in 0..r {
        for j in 0..c {
            if i == 0 && j == 0 {
                continue;
            }

            candidates.push((arc[i][j], i + j));
        }
    }
    candidates.sort_unstable();
    candidates.reverse();
    // println!("{:?}", candidates);

    for c in &candidates {
        if c.1 <= d && (d - c.1) % 2 == 0 {
            println!("{}", c.0);
            return;
        }
    }
}
