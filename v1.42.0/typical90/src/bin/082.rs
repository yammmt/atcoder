// :fu: :fu: :fu: :fu: :fu: 21-07 難易度は妥当だろうが個人的に苦手 まったく頭回らない
// TODO: 類題も解き直す 本番でも死んだ覚え
// WA: r = 1000000000000000000
// 10^19 は u64 型であればギリギリ扱える

use proconio::input;

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
        l: u64,
        r: u64,
    }
    let d = 1_000_000_007;

    let mut ans = 0;
    let two_inv = repeat_square(2, d - 2, d);
    // i + 1 桁の数が何回書かれるか
    for i in 1..20 {
        let ll = l.max(10u64.pow(i - 1));
        let rr = r.min(10u64.pow(i) - 1);
        // println!("{} {}", ll , rr);
        // rr 超えたら break しても
        if ll > rr {
            continue;
        }

        // println!("{} {}", ll , rr);
        let ll = ll % d;
        let rr = rr % d;
        ans = (ans + ((((ll + rr) * (rr + d - ll + 1)) % d * two_inv) % d) * i as u64 % d) % d;
        // ans = (ans + i as u64 * (ll + rr) * (rr - ll + 1) / 2);
    }

    println!("{}", ans);
}
