// 同方針: https://atcoder.jp/contests/abc250/editorial/3950

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    // q も素数と読んでよい？
    // N の三乗根以下の数を集めて, それぞれを q と仮置きする
    // p の値が一意に定まれば pass

    // N の三乗根の数
    let mut q_max: usize = 1;
    while q_max.pow(3) <= n {
        q_max += 1;
    }

    // N の三乗根以下の素数判定
    let mut is_prime = vec![true; q_max + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=q_max {
        if !is_prime[i] {
            continue;
        }

        let mut j = i + i;
        while j <= q_max {
            is_prime[j] = false;
            j += i;
        }
    }

    let mut prime_num = vec![0; q_max + 1];
    for i in 2..=q_max {
        prime_num[i] = if is_prime[i] {
            prime_num[i - 1] + 1
        } else {
            prime_num[i - 1]
        };
    }

    // ひとつひとつ数え上げると 10^18 / 2^3 で TLE する
    // 割った数以下の素数の数を予め線形時間で求めておく
    let mut ans = 0;
    for q in 2..=q_max {
        if !is_prime[q] {
            continue;
        }

        let qqq = q.pow(3);
        let p_max = n / qqq;
        ans += prime_num[p_max.min(q - 1)];
    }

    println!("{ans}");
}
