// :fu:
// 数問

// 0 と生整数 a の gcd は a になる

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    let mut gcd_l = vec![0; n + 1];
    let mut gcd_r = vec![0; n + 1];
    for i in 1..n + 1 {
        gcd_l[i] = gcd(gcd_l[i - 1], an[i - 1]);
    }
    for i in (0..n).rev() {
        gcd_r[i] = gcd(gcd_r[i + 1], an[i]);
    }
    // println!("l: {:?}", gcd_l);
    // println!("r: {:?}", gcd_r);

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(gcd(gcd_l[i], gcd_r[i + 1]));
    }
    println!("{}", ans);
}
