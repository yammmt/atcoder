use proconio::fastout;
use proconio::input;

const INF: isize = isize::MAX / 2;

fn upper_bound(a: isize, dp: &[isize]) -> usize {
    let mut fail = -1;
    let mut pass = dp.len() as isize;
    while pass - fail > 1 {
        let mid = ((pass + fail) / 2) as usize;
        if dp[mid] > a {
            pass = mid as isize;
        } else {
            fail = mid as isize;
        }
    }

    pass as usize
}

#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
        mut an: [isize; n],
    }
    an.reverse();
    for i in 0..n {
        an[i] -= i as isize;
    }

    // 最長増加部分列, LIS

    let mut dp = vec![INF; n + 1];
    for &a in &an {
        if !(a >= 0 && a <= (p as isize - n as isize + 1)) {
            // 必ず置き換えなくてはならないため, DP しない.
            // 右端の値について a+n-1 <= p <=> a <= p-n+1
            //   右側に n-1 個並べる領域が必要
            // LIS は置き換えなくて良い部分列の扱い
            continue;
        }

        let i = upper_bound(a, &dp);
        dp[i] = a;
    }
    // println!("{dp:?}");

    for i in (0..n + 1).rev() {
        if dp[i] != INF {
            println!("{}", n - i - 1);
            return;
        }
    }

    // 全要素が点数範囲外だとここを通る
    println!("{n}");
}
