// :fu: :fu: 21-12
// https://atcoder.jp/contests/abc180/editorial/219

use proconio::input;

fn main() {
    input! {
        x: i128,
        y: i128,
        a: i128,
        b: i128,
    }

    // 数式立てて O(1) で解こうとすると log が出てくる
    // 制約より x < y
    let mut strength = x;
    let mut exp = 0;
    while strength * a < y && strength * a < strength + b {
        strength *= a;
        exp += 1;
    }

    println!("{}", (exp + (y - 1 - strength) / b));
}
