// :fu: 21-06 もたつく数問 (25min)

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i64,
    }

    let mut ans = HashSet::new();
    let mut cur_n = 1;
    while cur_n * cur_n <= 2 * n {
        if 2 * n % cur_n == 0 {
            // a * b = 2 * n
            let a = cur_n;
            let b = 2 * n / a;

            // 項数が a
            if (b - a + 1) % 2 == 0 {
                ans.insert((a, (b - a + 1) / 2));
            }
            // 項数が b
            if (a - b + 1) % 2 == 0 {
                ans.insert((b, (a - b + 1) / 2));
            }
        }

        cur_n += 1;
    }

    // println!("{:?}", ans);
    println!("{}", ans.len());
}
