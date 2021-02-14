// 数問
// https://yamakasa.net/atcoder-arc-004-b/

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dn: [i64; n],
    }
    if n == 1 {
        println!("{}", dn[0]);
        println!("{}", dn[0]);
        return;
    }

    dn.sort_unstable();
    let ans1 = dn.iter().sum::<i64>();
    println!("{}", ans1);
    let ans2 = if ans1 - dn[n - 1] >= dn[n - 1] {
        0
    } else {
        dn[n - 1] - (ans1 - dn[n - 1])
    };
    println!("{}", ans2);
}
