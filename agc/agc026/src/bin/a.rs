// 3min

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
    }

    let mut ans = 0;
    for i in 1..n {
        if an[i] == an[i - 1] {
            an[i] = if i == n - 1 {
                an[i - 1]
            } else {
                an[i + 1] - 1
            };
            ans += 1;
        }
    }
    println!("{}", ans);
}
