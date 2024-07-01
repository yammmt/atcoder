use proconio::fastout;
use proconio::input;

fn ari_sum(b: u128, e: u128) -> u128 {
    (b + e) * (e - b + 1) / 2
}

#[fastout]
fn main() {
    input! {
        n: u128,
    }

    // 貪欲に長いものから順に買っていけばよい
    // 買った長さの合計がほしい長さの合計と一致するとは限らない？
    // 最安でない解は [1, n] を全部買うもの
    // ここから短い側のものを n+1 を切ることで可能な限り代替する, でよさそう
    // とすると sum[1, m] <= n+1 を満たす最大の m を求めればよい

    let mut pass = 0;
    let mut fail = n + 2;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        if ari_sum(1, mid) <= n + 1 {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", n + 1 - pass);
}
