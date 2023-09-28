use proconio::input;
use std::collections::HashSet;

fn ncr(n: usize, r: usize) -> usize {
    if r == 0 || n == r {
        1
    } else if n < r {
        0
    } else if r == 1 || n - r == 1 {
        n
    } else {
        // r とは
        n * (n - 1) / 2
    }
}

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let mut is_prime = vec![true; r + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 0..r {
        if !is_prime[i] {
            continue;
        }

        let mut j = 2 * i;
        while j <= r {
            is_prime[j] = false;
            j += i;
        }
    }
    // println!("{:?}", is_prime);

    // 最大公約数が 2 以上となる数の組を数え上げる
    // 自身の素因数を Set でもって自身より後ろの値と比べていけば良いが, そのままでは計算量がだめそう
    // 最大公約数が 2 となる組, 3 となる組…を数えていっても良いが, 愚直に数えると総数が大きすぎてだめそう
    // そもそも最大公約数が x であると高速に探せるか？
    // 下手に数えると例えば (15, 30) の組は素数 3/5 で二度数えられて詰む
    // 5 の倍数であり 3 の倍数でない, という角度からなら数えられる？

    // あるいは数え下げ方針として全通りから互いに素となる組の数を引く
    // こちらの方法でも楽して Set で数えようとすると TLE

    // 範囲全体に素数を覚えさせてもだめそう

    // [2, R] の範囲で全数の約数の個数を数えて包除原理でうまいことする？
    // 最大公約数が 5 以上となる個数 = 5 で割れる数の個数 - (2 * 5 で割れる数の個数 + 3 * 5 で割れる数の個数)
    // これで TLE しない？
    // 素数の個数は 10^6 までで 78,498 個らしく計算量としては危険
    let mut ans = 0usize;
    let mut appeared_primes = HashSet::new();
    for i in 0..r {
        if 2 * i > r {
            break;
        } else if !is_prime[i] {
            continue;
        }

        println!("i: {}", i);
        let mut cur = r / i - (l - 1) / i;
        for v in &appeared_primes {
            cur -= r / (*v * i) - (l - 1) / (*v * i);
        }
        println!("  cur1: {}", cur);
        ans += 2 * ncr(cur, 2);
        // TODO: **重複を避けつつ** 自身の倍数となる組数を引くつもりがこれだとだめそう
        let in_than_l = i * ((l + i - 1) / i);
        println!("  {}",in_than_l);
        if 2 * in_than_l <= r {
            let mut cur = 0;
            let mut j = 2 * in_than_l;
            while j <= r {
                cur += 1;
                j += in_than_l;
            }
            println!("  cur2: {}", cur);
            ans -= 2 * ncr(cur, 2);
        }

        appeared_primes.insert(i);
    }

    println!("{}", ans);
}
