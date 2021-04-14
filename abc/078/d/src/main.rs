// 知ってしまうと解き直してもというお気持ち

use proconio::input;

fn main() {
    input! {
        n: usize,
        _z: i64,
        w: i64,
        an: [i64; n],
    }

    println!(
        "{}",
        if n < 2 {
            (an[n - 1] - w).abs()
        } else {
            ((an[n - 1] - w).abs()).max((an[n - 1] - an[n - 2]).abs())
        }
    );
}
