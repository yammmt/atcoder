// :fu:

// 登録者と解答提出者の差が凄まじい
// m 進数で下二桁と考えると mod m^2 に落とし込める
// 公式解説は n 乗計算の高速化に触れていないが

use proconio::input;

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

fn main() {
    input! {
        n: u64,
        m: u64,
    }
    println!(
        "{}",
        repeat_square(10, n, m * m) / m
    );
}
