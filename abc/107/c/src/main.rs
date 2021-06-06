// 複雑でない境界条件 時間をかけたくない問題

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xn: [i64; n],
    }

    let mut ans = std::i64::MAX / 2;
    for i in 0..n - k + 1 {
        // println!("{}", i);
        let left = xn[i];
        let right = xn[i + k - 1];
        if left * right < 0 {
            ans = ans.min(right - left + right.min(left.abs()));
        } else if left < 0 {
            ans = ans.min(left.abs());
        } else {
            ans = ans.min(right);
        }
    }

    println!("{}", ans);
}
