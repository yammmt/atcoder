// 10min 1WA
// ans が i32 となってしまっていた
// n = 1 がテストケースにないので嘘解が通る…が n = 1 時は 0? 1?

use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }

    // if n == 1 {
    //     println!("1");
    //     return;
    // }

    let mut ans = 0u64;
    let mut streak = 1;
    for i in 1..n {
        match an[i].cmp(&an[i - 1]) {
            Ordering::Greater => streak += 1,
            _ => {
                ans += (streak * streak + streak) / 2;
                streak = 1;
            }
        }
        if i == n - 1 {
            ans += (streak * streak + streak) / 2;
        }
    }
    println!("{}", ans);
}
