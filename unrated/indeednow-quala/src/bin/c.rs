// テストケース非公開

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut sn: [i64; n],
        q: usize,
        kq: [usize; q],
    }
    sn = sn.iter().filter(|&s| *s > 0).copied().collect::<Vec<i64>>();
    sn.sort();

    for k in &kq {
        if *k >= sn.len() {
            println!("0");
        } else {
            // 最小値を出すため一つ前の要素 +1 にする
            // さもなくば 16WA
            println!("{}", sn[sn.len() - *k - 1] + 1);
        }
    }
}
