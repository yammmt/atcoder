// :fu:

// mod p の p が合成数となり得るためフェルマーの小定理では WA

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// m, n が互いに素であるときに mx + ny = 1
fn extgcd(mut a: i64, m: i64) -> u64 {
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u as u64
}

fn main() {
    input! {
        t: usize,
        nskt: [(u64, u64, u64); t],
    }

    for nsk in &nskt {
        let mut a = nsk.2;
        let mut b = nsk.0 - nsk.1;
        let mut m = nsk.0;
        // println!("a: {}", a);
        // println!("b: {}", b);
        // println!("m: {}", m);

        // https://高校数学.net/goudoushiki-houteishiki/
        // ax ≡ b (mod m)
        // gcd(a, m) == 1 or b % gcd(a, m) == 0 の場合に解がある
        // 前者は明らかに後者を満たす
        let d = gcd(a, m);
        if b % d != 0 {
            println!("-1");
            continue;
        }
        // 公約数を取り除かなければ最短でない解を引く (サンプル最後)
        let d = gcd(d, b);
        a /= d;
        b /= d;
        m /= d;

        println!(
            "{}",
            (extgcd(a as i64, m as i64) * b) % m
        );
    }
}
