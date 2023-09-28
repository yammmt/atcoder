// 8min
// 制約が小さいので一つずつ数字足していっても十分に間に合う

use proconio::input;

fn main() {
    input! {
        n: u64,
        k: usize,
        dk: [u64; k],
    }

    let mut ans = n;
    loop {
        let mut pass = true;
        let mut cur = ans;
        while cur > 0 {
            if dk.contains(&(cur % 10)) {
                pass = false;
                break;
            }

            cur /= 10;
        }

        if pass {
            println!("{}", ans);
            return;
        }

        ans += 1;
    }
}
