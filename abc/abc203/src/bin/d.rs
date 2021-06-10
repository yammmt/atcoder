// 二次元累積和

// http://sigma425.hatenablog.com/entry/2015/02/12/145854
// これでも解けそうだが Rust には最大値を簡単に拾う方法がない
// `iter` で最小値は拾えるので small 側には負にした値を入れておく？
// 結果として相当の重実装となってしぬ

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ann: [[i64; n]; n],
    }

    // 愚直に全通り試すと O(NKK) で計算回数が最大 5.0x10^8 回ほどとなり微妙そう
    // k^2 個の数の中央値が A 以下である場合には A 以下の数が k^2/2 + 1 個以上
    // あるいは A より大きい数が k^2/2 + 1 個未満
    let mut pass = 1_000_000_000;
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        // println!("{} {} => {}", pass, fail, mid);
        // 累積和テーブル
        let mut cusum = vec![vec![0i64; n + 1]; n + 1];
        for i in 0..n {
            for j in 0..n {
                cusum[i + 1][j + 1] = cusum[i + 1][j] + cusum[i][j + 1] - cusum[i][j];
                if ann[i][j] > mid {
                    cusum[i + 1][j + 1] += 1;
                }
            }
        }

        // 検討
        let mut cleared = false;
        'outer: for si in 0..n - k + 1 {
            for sj in 0..n - k + 1 {
                let gi = si + k;
                let gj = sj + k;
                // println!("{} {}", si, sj);
                // println!("{} {}\n", gi, gj);
                let over_num = cusum[gi][gj] - cusum[gi][sj] - cusum[si][gj] + cusum[si][sj];
                if over_num < (k * k / 2 + 1) as i64 {
                    cleared = true;
                    break 'outer;
                }
            }
        }

        if cleared {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
