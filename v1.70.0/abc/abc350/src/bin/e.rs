use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

fn calc(n: usize, a: usize, x: f64, y: f64, memo: &mut HashMap<usize, f64>) -> f64 {
    if let Some(&ret) = memo.get(&n) {
        ret
    } else {
        let cur = (x + calc(n / a, a, x, y, memo)).min(
            6.0 * y / 5.0
                + calc(n / 2, a, x, y, memo) / 5.0
                + calc(n / 3, a, x, y, memo) / 5.0
                + calc(n / 4, a, x, y, memo) / 5.0
                + calc(n / 5, a, x, y, memo) / 5.0
                + calc(n / 6, a, x, y, memo) / 5.0,
        );
        memo.insert(n, cur);
        cur
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        x: f64,
        y: f64,
    }

    // 愚直にやると, 手を固定して全通りで期待値を計算する
    //   手の取り方を XXX, XXY, XYY, YYY, ..., とするような
    // しかしこれだと TLE
    // あるいは, DP としてゴールから操作回数をカウントアップすると O(N) で TLE

    let mut expected_values = HashMap::new();
    // (数値, 支払金額の期待値)
    expected_values.insert(0, 0.0);

    println!("{}", calc(n, a, x, y, &mut expected_values));
}
