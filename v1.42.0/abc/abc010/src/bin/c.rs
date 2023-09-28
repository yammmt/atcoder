// テストケースは計算誤差甘かったらしい

use proconio::input;

fn main() {
    input! {
        txya: (i64, i64),
        txyb: (i64, i64),
        t: i64,
        v: i64,
        n: usize,
        xyn: [(i64, i64); n],
    }

    for xy in &xyn {
        let t1 = (txya.0 - xy.0) * (txya.0 - xy.0) + (txya.1 - xy.1) * (txya.1 - xy.1);
        let t2 = (txyb.0 - xy.0) * (txyb.0 - xy.0) + (txyb.1 - xy.1) * (txyb.1 - xy.1);
        let left = v * t * v * t - t1 - t2;
        if left <= 0 {
            continue;
        }

        if left * left >= 4 * t1 * t2 {
            println!("YES");
            return;
        }
    }

    println!("NO");
}
