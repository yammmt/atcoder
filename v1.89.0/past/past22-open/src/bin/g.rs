use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            n: u64,
            a: u64,
            b: u64,
        }

        // n が大きいので手前から一つずつ足していく方針では TLE
        // レベルを m 回上げるに必要な宝石は, B 部分は単純に掛けるだけ
        // A 部分は (1 + 2 + ... + m) * A で出せる
        // つまりは二分探索である

        let mut pass = 0;
        let mut fail = 2_000_000_001;
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            let gems_a = a * (mid - 1);
            let gems_b = b;
            if gems_a + 2 * gems_b <= 2 * n / mid {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        // いくつ上げられるか
        println!("{}", pass);
    }
}
