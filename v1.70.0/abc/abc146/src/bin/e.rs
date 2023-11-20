use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    let mut cusum = vec![0; n + 1];
    for i in 1..=n {
        cusum[i] = (cusum[i - 1] + an[i - 1] - 1) % k;
    }

    let mut ans = 0u64;
    let mut counter = HashMap::new();
    counter.insert(0, 1);
    for i in 1..=n {
        // println!("{i}");
        if i >= k {
            let cnt = counter.get(&cusum[i - k]).unwrap();
            // println!("  drop {}", cusum[i - k]);
            counter.insert(cusum[i - k], cnt - 1);
        }
        if let Some(cur) = counter.get(&cusum[i]) {
            // println!("  += {cur}");
            ans += cur;
        }
        let cnt = counter.entry(cusum[i]).or_insert(0);
        *cnt += 1;
    }

    println!("{ans}");
}
