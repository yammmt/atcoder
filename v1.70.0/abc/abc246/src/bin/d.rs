use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let mut ans = u64::MAX;
    let mut a = 0;
    while a * a * a <= n {
        if a * a * a == n {
            ans = ans.min(4 * (a * a * a));
            break;
        }

        // a を固定し, n 以上である最小の値を取る b を二分探索する
        // 探索の初期値を適当に大きくすると 10^18^3 が発生して終わる
        let mut pass = 1_000_001u64;
        let mut fail = 0;
        while pass - fail > 1 {
            let b = (pass + fail) / 2;
            // println!("{a}, {b}");
            let cur = (a + b) * (a * a + b * b);
            if cur >= n {
                pass = b;
            } else {
                fail = b;
            }
        }

        ans = ans.min((a + pass) * (a * a + pass * pass));
        a += 1;
    }

    println!("{ans}");
}
