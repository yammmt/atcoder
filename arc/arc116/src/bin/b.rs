// :fu: 21-04 立式が遅い

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [u64; n],
    }
    let d = 998_244_353;

    an.sort();
    an.reverse();
    let mut ans = 0u64;
    let mut cur = 0;
    let mut muled_sum = 0;
    for i in 0..n {
        muled_sum = (muled_sum * 2) % d;
        if i > 1 {
            muled_sum = (muled_sum + an[i - 2]) % d;
            cur = (cur + muled_sum) % d;
        }
        cur = (cur + an[i]) % d;
        // println!("ai: {}", an[i]);
        // println!("cur: {}", cur);
        // println!("muled_sum: {}", muled_sum);
        ans = (ans + an[i] * cur) % d;
    }

    println!("{}", ans);
}
