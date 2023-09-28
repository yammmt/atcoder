// :fu: :fu: 21-07 マンハッタン距離は 45 度回転

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xyn_before: [(i64, i64); n],
        qq: [usize; q],
    }

    let xyn: Vec<(i64, i64)> = xyn_before
        .iter()
        .map(|xy| (xy.0 - xy.1, xy.0 + xy.1))
        .collect();
    let mut xmax = std::i64::MIN;
    let mut xmin = std::i64::MAX;
    let mut ymax = std::i64::MIN;
    let mut ymin = std::i64::MAX;
    for xy in &xyn {
        xmax = xmax.max(xy.0);
        xmin = xmin.min(xy.0);
        ymax = ymax.max(xy.1);
        ymin = ymin.min(xy.1);
    }

    for q in &qq {
        println!(
            "{}",
            (xyn[q - 1].0 - xmax).abs().max(
                (xyn[q - 1].0 - xmin)
                    .abs()
                    .max((xyn[q - 1].1 - ymax).abs().max((xyn[q - 1].1 - ymin).abs()))
            )
        );
    }
}
