// 遅い

use proconio::input;

fn main() {
    input! {
        n: i128,
        an: [i128; n],
    }
    let mut ans = 0;
    let mut wa = 0;
    for a in &an {
        ans += (n - 1) * (a * a);
        wa += 2 * a;
    }
    // println!("{}", ans);
    for i in 0..n {
        wa -= 2 * an[i as usize];
        ans -= an[i as usize] * wa;
    }

    println!("{}", ans);
}
