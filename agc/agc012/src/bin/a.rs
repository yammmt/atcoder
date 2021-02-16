// 7min (1WA 1.5min)
// WA: 雑に真ん中の値だけ取ってた

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; 3 * n],
    }
    an.sort();
    let mut ans = 0;
    for (i, a) in an.iter().skip(n).enumerate() {
        if i % 2 == 0 {
            ans += *a;
        }
    }
    println!("{}", ans);
}
