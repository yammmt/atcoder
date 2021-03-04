// 灰…？

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        _c: i64,
        k: usize,
    }
    let ans = if k % 2 == 0 {
        a - b
    } else {
        b - a
    };
    if ans.abs() > 10i64.pow(18) {
        println!("Unfair");
    } else {
        println!("{}", ans);
    }
}
