use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        am: [u64; m],
    }
    let d = 10u64.pow(9) + 7;

    let mut broken = HashSet::new();
    for a in &am {
        broken.insert(*a as usize);
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..n + 1 {
        if broken.contains(&i) {
            continue;
        }

        // println!("i: {}", i);
        if i > 0 {
            // println!("i - 1: {}", i - 1);
            dp[i] = (dp[i] + dp[i - 1]) % d;
        }
        if i > 1 {
            // println!("i - 2: {}", i - 2);
            dp[i] = (dp[i] + dp [i - 2]) % d;
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[n]);
}
