// :fu: :fu: :fu: 21-12 数問 とても嫌い こんなん ABC では青 Diff

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    // 答えが 1    になる => 分母が N / 2 より大きい
    // 答えが 2 以下になる => 分母が N / 3 より大きい
    // 答えが x 以下になる => 分母が N / (x + 1) より大きい
    // が, これで数えると分母を 2 から N / 2 まで試して TLE

    // 制約より約数列挙らしき雰囲気がある
    // 2^40 > 10^12 より N の約数の個数は最大で 40 個程度
    // だが足す値は約数以外でも変わる場合があり高速に計算できず (ex. n = 10)

    let mut ans = 0;
    let mut already_counted = 0u64;
    for i in 2..n + 1 {
        println!("i: {}", i);
        if i * i > n {
            // 残り全部を 1 として足す？
            break;
        }
        let no_more_than = (n + i - 1) / i;
        ans += (i - 1) * (no_more_than - already_counted);
        already_counted += no_more_than;
        println!("  ans: {}", ans);
    }

    println!("{}", ans);
}
