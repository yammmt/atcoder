use proconio::fastout;
use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[fastout]
fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
    }

    // 全部足して an と bm を引いて ab 公倍数を足す

    let ap_sum = |last_term| last_term * (1 + last_term) / 2;

    let mut n_sum = n * (1 + n) / 2;
    let a_sum = a * ap_sum(n / a);
    let b_sum = b * ap_sum(n / b);
    let ab = lcm(a, b);
    let ab_sum = ab * ap_sum(n / ab);

    n_sum -= a_sum;
    n_sum -= b_sum;
    n_sum += ab_sum;

    println!("{n_sum}");
}
