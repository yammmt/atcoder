// 3.5min

use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    let mut ans = std::u64::MAX / 2;
    let a0 = a / 2;
    let a1 = (a + 1) / 2;
    ans = ans.min((a1 - a0) * b * c);
    let b0 = b / 2;
    let b1 = (b + 1) / 2;
    ans = ans.min((b1 - b0) * a * c);
    let c0 = c / 2;
    let c1 = (c + 1) / 2;
    ans = ans.min((c1 - c0) * a * b);

    println!("{}", ans);
}
