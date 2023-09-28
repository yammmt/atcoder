// 最小全域木/クラスカル法らしい 未履修

use proconio::input;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: u128,
        m: usize,
        mut acm: [(u128, u128); m],
    }
    // A_i <= n - 1

    // 連結にできる十分条件は A_M の最大公約数が 1 となる or N と互いに素な A_M が存在する？
    // 最小値は最安操作を可能な限り繰り返して最後にマージ？
    // どうせ一度に繋げられる点数は二点だけなのだから N 点を繋ぐために必要な操作回数は N - 2 回
    let mut cur_gcd = acm[0].0;
    let mut each_prime = false;
    for ac in &acm {
        cur_gcd = gcd(cur_gcd, ac.0);
        if gcd(n, ac.0) == 1 {
            each_prime = true;
        }
    }

    if cur_gcd != 1 && !each_prime {
        println!("-1");
        return;
    }

    let mut ans = std::u128::MAX / 4;
    acm.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    // println!("{:?}", acm);
    if cur_gcd == 1 {
        // ここが怪しい "6 1 5 1" で WA
        let cheapest_op_num = n - acm[0].0;
        // println!("  num: {}", cheapest_op_num);
        ans = acm[0].1 * cheapest_op_num;
        // println!("  ans: {}", ans);
        for ac in &acm {
            if gcd(acm[0].0, ac.0) == 1 {
                ans += ac.1 * (n - 1 - cheapest_op_num);
                break;
            }
        }
    }

    for ac in &acm {
        // println!("{:?}", ac);
        if gcd(ac.0, n) == 1 {
            // println!(" x");
            ans = ans.min(ac.1 * (n - 1));
            break;
        }
    }

    println!("{}", ans);
}
