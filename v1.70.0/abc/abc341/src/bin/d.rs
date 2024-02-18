use proconio::fastout;
use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if a % b == 0 {
        return b;
    }

    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[fastout]
fn main() {
    input! {
        n: u64,
        m: u64,
        k: u64,
    }

    // n or m の片方のみで割り切れる数を k 個以上含むのが pass
    // 初期値は雑
    let mut fail = 0;
    let mut pass = u64::MAX / 2;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let by_n = mid / n;
        let by_m = mid / m;
        let by_nm = mid / lcm(n, m);
        if by_n + by_m - 2 * by_nm < k {
            fail = mid;
        } else {
            pass = mid;
        }
    }

    println!("{pass}");
}
