// :fu: :fu: :fu: :fu: :fu: 21-07 Rust の癖で実装が苦しくなるしそもそも実装が重め
// https://drken1215.hatenablog.com/entry/2020/01/22/071000

use proconio::input;
use std::collections::HashMap;

const A_MAX: usize = 1_000_001;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: usize, p: usize, m: usize) -> usize {
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

fn ninv(n: usize, p: usize) -> usize {
    repeat_square(n, p - 2, p)
}

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }
    let d = 1_000_000_007;

    // 素数判定
    let mut is_prime = vec![true; A_MAX];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 0..A_MAX {
        if !is_prime[p] {
            continue;
        }

        let mut i = 2 * p;
        while i < A_MAX {
            is_prime[i] = false;
            i += p;
        }
    }

    // 全数の最小公倍数を素因数を保持する形で記憶する
    let mut prime_num = vec![0; A_MAX];
    for a in &an {
        let mut cur = *a;
        let mut cur_prime = HashMap::new();
        let mut i = 2;
        while cur > 1 && i * i <= *a {
            if !is_prime[i] {
                i += 1;
                continue;
            }

            if cur % i == 0 {
                let cnt = cur_prime.entry(i).or_insert(0);
                *cnt += 1;
                cur /= i;
            } else {
                i += 1;
            }
        }
        if cur > 1 {
            cur_prime.insert(cur, 1);
        }

        for (k, v) in &cur_prime {
            prime_num[*k] = prime_num[*k].max(*v);
        }
    }

    let mut lcm = 1;
    for (i, v) in prime_num.iter().enumerate() {
        lcm = (lcm * repeat_square(i, *v, d)) % d;
    }
    let lcm = lcm;

    let mut ans = 0;
    for a in &an {
        ans = (ans + lcm * ninv(*a, d)) % d;
    }

    println!("{}", ans);
}
