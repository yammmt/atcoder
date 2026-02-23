use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // 真ん中を固定すると, 左/右にどの要素がいくつあるのかは数えられる
    // でも愚直に数え上げようには, すべての登場した数に対してループ回すことになり TLE
    // 最初に全要素分の組み合わせ数えて, 以降差分を O(1) で計算を N 回かしら

    // all/left から動的に right 計算した方が高速なはず
    let mut counter_right = HashMap::new();
    for &a in &an {
        let k = counter_right.entry(a).or_insert(0);
        *k += 1;
    }

    let mut ans = 0usize;
    let mut cur = 0;
    let mut counter_left = HashMap::new();
    for (i, a) in an.iter().enumerate() {
        if i == n - 1 {
            continue;
        }

        let cal = *counter_left.get(a).unwrap_or(&0);
        let car = *counter_right.get(a).unwrap_or(&0);

        // 今の要素分を差し引く
        ans += cur - cal * car;

        // 左に入る分を足す
        cur += (cal + 1) * (car - 1);
        cur -= cal * car;

        let k = counter_right.entry(*a).or_insert(0);
        *k -= 1;
        let k = counter_left.entry(*a).or_insert(0);
        *k += 1;
    }

    println!("{ans}");
}
