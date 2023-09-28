// 29min -> 16min

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut ans = 0;
    // お気に入りの数 m は N の約数 r について N / r - 1
    // かつ r は余りでもあるので約数ともなる m について r < m
    for r in 1..n {
        if r * r > n {
            break;
        } else if n % r != 0 {
            continue;
        }

        let m = n / r - 1;
        if n / m == n % m {
            ans += m;
        }
        // N を m で割った余りを r とすると明らかに r < m だが
        // 約数の反対側 (N/m) はこの条件を満たさないらしい
    }

    println!("{}", ans);
}
