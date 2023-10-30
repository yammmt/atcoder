// Editorial の "ここで、", "この問題は" の飛躍が辛い, 解答例もなし, 個人解説も皆無

// [0, 4] の区間和がこれ, [0, 10] の区間和がそれ, となると
// [0, 10] のそれから [0, 4] のこれを引いた結果の絶対値が x 以下ならよい (累積和の基礎)
// 今回は i, j (上の区間では 4, 10) の順序を気にすると O(N^2) が入ってしまうので
// これを高速化したい. 高速化として尺取りにせよ二分探索にせよ, まずは値がソートされていることが
// 必要になる. 値をソートして扱うためには, i, j の順序を無視する (i!=j).
// (i, j) の順序を無視すると, 出現する累積和の要素数は (i, j), (j, i) が区別できない分で
// 本来の二倍になる. 本当に？

// sort して尺取りで k 二倍部分いらなくない？
// 累積和が [0, 4, 3, -3, 7, -4] として sort して [-4, -3, 0, 3, 4, 7]
// 尺取りで左から詰めて (i, j) が登場するのは高々一回であって (i, j) と (j, i) が
// 重複してカウントされることはないはず

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i64; n],
    }

    let mut cusum = vec![0; n + 1];
    for i in 1..n + 1 {
        cusum[i] = cusum[i - 1] + an[i - 1];
    }
    cusum.sort_unstable();

    // pass 以下の要素は k 個以上
    // fail を 0 とすると, サンプルは通るが答えが 0 となる場面で落ちる
    let mut pass = 3 * 10i64.pow(5) * 10i64.pow(9);
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mut cnt = 0;
        let mut r = 0;
        for l in 0..cusum.len() {
            // sort 済みだから abs 不要
            while r < cusum.len() && cusum[r] - cusum[l] <= mid {
                r += 1;
            }
            // 加算する数は r が範囲外に出ることを考える
            cnt += r - l - 1;
        }

        if cnt >= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{pass}");
}
