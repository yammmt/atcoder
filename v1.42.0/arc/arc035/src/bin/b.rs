// 21min
// ペナルティ計算でしくった

use proconio::input;

fn factorial(n: u64, d: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut ret = 1;
    for i in 1..n + 1 {
        ret = (ret * i) % d;
    }
    ret
}

fn main() {
    input! {
        n: usize,
        mut tn: [u64; n],
    }
    let d = 10u64.pow(9) + 7;

    tn.sort_unstable();
    let mut pena = 0;
    let mut tsum = 0;
    for t in &tn {
        // println!("pena: {}", pena);
        // println!("pena += {}", tsum + *t);
        pena = pena + tsum + *t;
        tsum += *t;
    }

    let mut ways = 1;
    let mut streak = 1;
    // println!("{:?}", tn);
    for (i, t) in tn.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if *t == tn[i - 1] {
            streak += 1;
        }

        if *t != tn[i - 1] || i == n - 1 {
            // println!("streak: {}", streak);
            ways = (ways * factorial(streak, d)) % d;
            streak = 1;
        }
    }

    println!("{}", pena);
    println!("{}", ways);
}
