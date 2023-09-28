// 6min 1WA

use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(u64, u64); n],
    }

    let mut ans = 0;
    for i in (0..n).rev() {
        let cur = abn[i].0 + ans;
        if cur % abn[i].1 != 0 {
            ans += abn[i].1 - (cur % abn[i].1);
        }
    }
    println!("{}", ans);
}
