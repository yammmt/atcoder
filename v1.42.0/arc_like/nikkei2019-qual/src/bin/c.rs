// 39min 1WA (27.5min) -> 4.5min

use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(i64, i64); n],
    }

    let mut scores = vec![];
    for (i, ab) in abn.iter().enumerate() {
        scores.push((ab.0 + ab.1, i));
    }
    scores.sort_unstable();
    scores.reverse();
    let mut tkhs = 0;
    let mut aoki = 0;
    for (i, s) in scores.iter().enumerate() {
        if i % 2 == 0 {
            tkhs += abn[s.1].0;
        } else {
            aoki += abn[s.1].1;
        }
    }

    println!("{}", tkhs - aoki);
}
