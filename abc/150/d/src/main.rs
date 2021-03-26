// :fu: 21-03 数問だが diff は過大評価 (本番でテストケース誤り)
// https://scrapbox.io/pocala-kyopro/D_-_Semi_Common_Multiple

use proconio::input;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn main() {
    input! {
        n: usize,
        m: i64,
        an: [i64; n], // even
    }

    // 愚直にやるなら最大要素の半公倍数を全列挙してふるいにかければ良さそうだがサンプル 3 見ると TLE
    // 10^9 級の素数を 10^5 回掛け算する可能性？
    let mut cur_lcm = 1;
    an.iter().for_each(|a| cur_lcm = lcm(cur_lcm, *a));
    for a in &an {
        if cur_lcm / 2 % *a == 0 {
            println!("0");
            return;
        }
    }
    println!("{}", (m + 1 - cur_lcm / 2 + cur_lcm - 1) / cur_lcm);
}
