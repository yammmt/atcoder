// 数問

use proconio::fastout;
use proconio::input;

const MOD: u64 = 1_000_000_007;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * repeat_square(n, p - 1, m)) % m
    }
}

fn ninv(n: u64, p: u64) -> u64 {
    repeat_square(n, p - 2, p)
}

#[fastout]
fn main() {
    input! {
        n: u64,
        k: u64,
    }

    // n 個のものを k 箇所に配る組み合わせとして (n+k-1)C(k-1)
    let mut molecule = 1;
    for i in 0..k {
        // println!("i: {i}, *= {}", n+k-1-i);
        molecule = (molecule * (n+k-1-i)) % MOD;
    }

    let mut denominator = 1;
    for i in 0..k {
        denominator = (denominator * (i + 1)) % MOD;
    }

    let denominator_inv = ninv(denominator, MOD);
    let ans = (molecule * denominator_inv) % MOD;

    println!("{ans}");
}
