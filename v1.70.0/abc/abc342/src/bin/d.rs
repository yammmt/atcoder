use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

const A_MAX: usize = 200_001;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // 2x3x3 と 2 を掛けると平方数になる
    // 全数を素因数分解して, 奇数回掛けた項のみを数えてやる
    // 計算量 O(N x sqrt(N)) で怪しい, map の計算量も考えるとだめな気がするが

    let mut ans = 0usize;
    let mut terms = vec![0; A_MAX];

    for (i, &a) in an.iter().enumerate() {
        if a == 0 {
            ans += i;
            terms[0] += 1;
            continue;
        }

        let mut divisors = HashMap::new();
        let mut aa = a;
        let mut j = 2;
        while aa > 1 && j * j <= a {
            if aa % j == 0 {
                aa /= j;
                let cnt = divisors.entry(j).or_insert(0);
                *cnt += 1;
            } else {
                j += 1;
            }
        }
        let cnt = divisors.entry(aa).or_insert(0);
        *cnt += 1;

        let mut cur = 1;
        for (k, v) in divisors {
            if v % 2 == 1 {
                cur *= k;
            }
        }

        ans += terms[cur];
        ans += terms[0];
        terms[cur] += 1;
    }

    println!("{ans}");
}
