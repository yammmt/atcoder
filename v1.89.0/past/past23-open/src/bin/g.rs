use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        an: [usize; n],
        lrq: [(Usize1, Usize1); q],
    }

    // 一周の和を計算し全部含まれる分だけまとめて足して, 端数を区間和
    // 最悪ケースでは 10^6 が 10^12 個続いて 10^18, u64 範囲内
    // 場合分けがシンプルに嫌, かなり細かい

    let mut cusum = vec![0; n + 1];
    let mut cur = 0;
    for i in 0..n {
        cur += an[i];
        cusum[i + 1] = cur;
    }

    let cusum_to_n = |r: usize| cusum[n] * (r / n) + cusum[r % n];

    for (l, r) in lrq {
        let ans = cusum_to_n(r + 1) - cusum_to_n(l);
        println!("{ans}");
    }
}
