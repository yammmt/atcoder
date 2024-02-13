// D より難しくないか

use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

fn solve(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 2;
    }

    if let Some(&v) = memo.get(&n) {
        v
    } else {
        let v_lower = solve(n / 2, memo);
        let v_upper = solve((n + 1) / 2, memo);
        let cur = v_lower + v_upper + n;
        memo.insert(n, cur);
        cur
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut memo = HashMap::new();
    println!("{}", solve(n, &mut memo));
}
