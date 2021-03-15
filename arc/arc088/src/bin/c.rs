// 5min

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() {
    input! {
        x: u64,
        y: u64,
    }
    let mut ans = vec![x];
    let mut cur = 2 * x;
    while cur <= y {
        ans.push(cur);
        cur = lcm(cur, ans[ans.len() - 2]);
        if cur == ans[ans.len() - 1] {
            cur *= 2;
        }
        // println!("{:?}", ans);
    }
    println!("{}", ans.len());
}
