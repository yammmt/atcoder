// Yokan Party っぽさがあるのだが, 同じようにコードを書くと通らないし想定難易度も違いそう
// "切断後の一番短い棒の長さをできる限り長くします" の解釈がよくない？
// 長さ制約に上限下限のどちらもあるあたりで読めていない気がする
// 下限超えた時点で切る, をすると, 下限未達前に一気に上限を超える項を救えない可能性があるから？
// っぽい, するとたしかにこれは DP だし, 切れる区間の更新にいもす法を使いたくなる

// 個々の要素は言われればわかるのだが, 重い (個人の実力の都合で) = 良問 for me

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        an: [usize; n],
    }
    // 累積和を考えるため, 左端からの位置で考える
    let mut sn = vec![0; n + 1];
    for i in 1..=n {
        sn[i] = sn[i - 1] + an[i - 1];
    }
    // println!("{:?}", sn);

    let mut pass = 0;
    let mut fail = l + 1;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;

        let mut imos = vec![0isize; n + 1];
        let mut imos_cur = 1;
        imos[1] = -1;
        let mut caterpillar_l = 0;
        let mut caterpillar_r = 0;
        for i in 0..n {
            imos_cur += imos[i];
            if imos_cur == 0 {
                // そこでは切れない, 0 未満にはならない
                continue;
            }

            // 今の切り込みで切った後の長さが mid 以上 L 以下となる範囲で切る
            // 尺取り
            // 累積和を別にとって二分探索で両端を求めることもできるが, より複雑な気配
            // loop の意図: sn[l]-sn[i] (長さ) が mid 以上となるまで
            while caterpillar_l <= n && sn[caterpillar_l] - sn[i] < mid {
                caterpillar_l += 1;
            }
            if caterpillar_l > n {
                continue;
            }

            imos[caterpillar_l] += 1;

            while caterpillar_r + 1 <= n && sn[caterpillar_r + 1] - sn[i] <= l {
                caterpillar_r += 1;
            }
            if caterpillar_r + 1 <= n {
                imos[caterpillar_r + 1] -= 1;
            }
        }

        imos_cur += imos[n];
        if imos_cur > 0 {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{pass}");
}
