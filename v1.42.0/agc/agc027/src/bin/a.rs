// 4.5min 問題セットの傾斜が鬼

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: i64,
        mut an: [i64; n],
    }
    an.sort();
    let mut ans = 0;
    for (i, a) in an.iter().enumerate() {
        x -= *a;
        if (i != n - 1 && x >= 0) || (i == n - 1 && x == 0) {
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
