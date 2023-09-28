// MBP で stack overflow
// WA: 剰余取る位置

use proconio::input;

fn tribonacchi(n: usize, mut memo: &mut Vec<i32>) -> i32 {
    if memo[n] != -1 {
        memo[n]
    } else if n < 3 {
        memo[n] = 0;
        0
    } else if n < 5 {
        memo[n] = 1;
        1
    } else {
        let t = (tribonacchi(n - 1, &mut memo)
            + tribonacchi(n - 2, &mut memo)
            + tribonacchi(n - 3, &mut memo))
            % 10007;
        memo[n] = t;
        t
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut vmemo = vec![-1; n + 1];
    println!(
        "{}",
        tribonacchi(n, &mut vmemo)
    );
}
