use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [isize; n],
    }

    // 尺取り, 境界条件が嫌い
    // 等差数列だから区間 [l, r] が条件を満たすならば [l, r-1] などもまとめて条件を満たす
    // [1, 3] が条件を満たすならば [1, 1], [1, 2], [1, 3], [2, 2], [2, 3], [3, 3] と
    // 6 つの数列が条件を満たす
    // 一般化すると, 区間幅の数, 区間幅の数-1, ... の数列の和だから等差数列の和の公式
    // ... とすると重複省く部分でバグり散らかす

    // editorial めっちゃ難しい書き方するな...

    // 区間 [l, r] が条件を満たすなら true を返す
    let right_extended = |l: usize, r: usize| {
        if r - l <= 1 {
            // [l, l] と [l, l+1] は条件を満たすので
            true
        } else {
            an[r] - an[r - 1] == an[l + 1] - an[l]
        }
    };

    let mut ans = 0;
    let mut right = 1;
    for left in 0..n {
        if right < left {
            right = left;
        }
        while right < n && right_extended(left, right) {
            right += 1;
        }

        // left を初項とする
        // right は条件を満たさない項であるため, 末項の取り方は right-left 通りとなる
        ans += right - left;
    }

    println!("{:?}", ans);
}
