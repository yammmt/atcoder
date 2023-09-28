// 数問

use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(i64, i64); n],
    }

    let mut an = abn.iter().map(|ab| ab.0).collect::<Vec<i64>>();
    an.sort_unstable();
    let mut bn = abn.iter().map(|ab| ab.1).collect::<Vec<i64>>();
    bn.sort_unstable();

    // X は整数だが中央値は整数とは限らない
    println!(
        "{}",
        if n % 2 == 0 {
            bn[n / 2] + bn[n / 2 - 1] - an[n / 2] - an[n / 2 - 1] + 1
        } else {
            bn[n / 2] - an[n / 2] + 1
        }
    );
}
