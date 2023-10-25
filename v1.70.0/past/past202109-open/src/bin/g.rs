use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        abcn: [(u64, u64, u64); n],
    }

    // この数列の中に x 以下の値は何個あるか, なら O(N) で一度分判定取れる
    // k 以下, を二分探索すると O(Nlog(a_sum)) となり間に合う？
    // k 最大値は 10^9 の配列が 10^5 個続いた場合で 10^14
    // 項の最大値は初項 10^9 から公差 10^9 が 10^9 個続いた場合

    // pass: 自身より小さい要素が k 個以上
    let mut pass = u64::MAX / 2;
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mut cur = 0;
        for abc in &abcn {
            let a = abc.0;
            let b = abc.1;
            let c = abc.2;

            // cur: 自身より小さい要素の数
            if mid < b {
                // do nothing
            } else if b + c * (a - 1) < mid {
                cur += a;
            } else {
                cur += (mid + c - b) / c;
            }
        }

        if cur >= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{pass}");
}
