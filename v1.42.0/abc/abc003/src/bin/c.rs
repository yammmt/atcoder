// 3min

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut rn: [u64; n],
    }
    rn.sort_unstable();
    let mut ans = 0.0;
    for i in rn.iter().skip(n - k) {
        ans = (ans + *i as f64) / 2.0;
    }
    println!("{}", ans);
}
