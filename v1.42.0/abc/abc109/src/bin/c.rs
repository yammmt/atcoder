// 6min

use proconio::input;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        x: i64,
        xn: [i64; n],
    }

    if n == 1 {
        println!("{}", (xn[0] - x).abs());
        return;
    }

    let mut xgcd = gcd((xn[0] - x).abs(), (xn[1] - x).abs());
    #[allow(clippy::needless_range_loop)]
    for i in 2..n {
        xgcd = gcd(xgcd, (xn[i] - x).abs());
    }
    println!("{}", xgcd);
}
