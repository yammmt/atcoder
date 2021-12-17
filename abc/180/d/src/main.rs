// :fu: :fu: 21-12 数問として O(1) で解いた場合の式が合わない
// https://atcoder.jp/contests/abc180/editorial/219

use proconio::input;

fn main() {
    input! {
        x: u128,
        y: u128,
        a: u128,
        b: u128,
    }

    // let a_num = b / ((a - 1) * a * x);
    // let strength = x * a_num * a;
    // let exp = a_num;

    let mut strength = x.min(y);
    let mut exp = 0;
    while strength * a < y && strength * a < strength + b {
        strength *= a;
        exp += 1;
    }

    println!("{}", exp + (y - 1 - strength) / b);
}
