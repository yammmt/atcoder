// 39min 1WA (27.5min)
// WA: 見当違いな上に実装も単純に誤っていたがサンプル通ってしまった

use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(i64, i64); n],
    }

    let mut absum = Vec::with_capacity(n);
    for (i, ab) in abn.iter().enumerate() {
        absum.push((ab.0 + ab.1, i));
    }
    absum.sort_unstable();
    absum.reverse();
    let mut a_pts = 0;
    let mut b_pts = 0;
    for i in 0..n {
        match i % 2 {
            0 => a_pts += abn[absum[i].1].0,
            1 => b_pts += abn[absum[i].1].1,
            _ => unreachable!(),
        }
    }
    println!("{}", a_pts - b_pts);
}
