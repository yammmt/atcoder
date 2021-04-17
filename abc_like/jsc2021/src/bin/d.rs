// :fu: :fu: :fu: 21-04 誤読と mod 計算ミスで 30min 以上

use proconio::input;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
    // println!("{} {} {}", n, p, m);
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * (repeat_square(n, p - 1, m) % m)) % m
    }
}

fn main() {
    input! {
        n: u64,
        p: u64,
    }
    let d = 1_000_000_007u64;

    // 要素の範囲は [1, p - 1]
    // n, p <= 10^9 より for ループ不可

    let mut ans = p - 1;
    ans = (ans * repeat_square(p - 2, n - 1, d)) % d;
    println!("{}", ans);
}
