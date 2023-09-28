// -*- coding:utf-8-unix -*-

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64, g: u64) -> u64 {
    a / g * b
}

fn main() {
    input! {
        n: u64,
        m: u64,
        s: String,
        t: String,
    }

    let vs: Vec<char> = s.chars().collect();
    let vt: Vec<char> = t.chars().collect();
    let g = gcd(n, m);
    let l = lcm(n, m, g);

    // 理解しやすいが計算量の増える別解: LCM(N/L, M/L) の倍数の位置すべてを確認する
    // https://betrue12.hateblo.jp/entry/2018/10/14/121018
    // GCD が分母にくる理由を端折らずに解説してくださる良心
    // https://microkents.hatenablog.com/entry/2018/12/30/145128
    // 双方の文字列が進んでいく上で進み幅 L/N と L/M の公倍数となる位置がぶつかる
    // 例えば s については LCM(L/N, L/M) 動くと公倍数となり、文字一つで動く量は L/N
    // LCM(L/N, L/M) / (L/N) = L^2/NM * N/L = L/M
    // L が既出なのでこのままでも AC だし、 LCM(N, M) = N*M/GCD(n, m) を代入すればこの式になる
    for i in 0..g {
        if vs[(i * (n / g)) as usize] != vt[(i * (m / g)) as usize] {
            println!("-1");
            return;
        }
    }

    println!("{}", l);
}
