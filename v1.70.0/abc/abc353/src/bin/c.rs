// 最大の苦手, 二分探索だと最早 500 点問題より難しい

use proconio::fastout;
use proconio::input;

const MOD: usize = 100_000_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
    }

    // 自身と足してオーバーフローしない分は愚直に全部足す,
    // オーバーフローする分は愚直に全部足してからオーバーフロー分を引く

    an.sort_unstable();
    let mut a_sum = an.iter().sum::<usize>();

    let mut ans = 0;
    for (i, &a) in an.iter().enumerate() {
        a_sum -= a;
        ans += a_sum;
        ans += a * (n - i - 1);

        // オーバーフローが始まる idx は必ず i + 1 以降
        // さもなくば同じ数が連続した場合の判定を誤る
        let mut lb = i;
        let mut ub = n;
        while ub - lb > 1 {
            let mid = (lb + ub) / 2;
            if a + an[mid] < MOD {
                lb = mid;
            } else {
                ub = mid;
            }
        }

        ans -= MOD * (n - ub);
    }

    println!("{ans}");
}
