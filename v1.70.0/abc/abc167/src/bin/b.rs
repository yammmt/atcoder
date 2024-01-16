use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        k: i64,
    }

    let mut ans = 0;
    let a_use = a.min(k);
    ans += a_use;
    let b_use = b.min(k - a_use);
    let c_use = c.min(k - a_use - b_use);
    ans -= c_use;

    println!("{ans}");
}
