// 29min

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut ans = 0;
    let mut p = 1;
    while p * p < n {
        if n % p == 0 {
            let m = p - 1;
            if !(m <= 1 || n / m != n % m) {
                ans += m;
            }

            if n / p != p {
                let m = n / p - 1;
                if !(m <= 1 || n / m != n % m) {
                    ans += m;
                }
            }
        }
        p += 1;
    }
    println!("{}", ans);
}
