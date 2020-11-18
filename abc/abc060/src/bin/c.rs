// 8min

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u64,
        tn: [u64; n],
    }

    let mut ans = 0;
    let mut expired = 0;
    for i in &tn {
        // println!("expired: {}", expired);
        if expired > *i {
            ans += t - (expired - *i);
        } else {
            ans += t;
        }
        expired = *i + t;
    }
    println!("{}", ans);
}
