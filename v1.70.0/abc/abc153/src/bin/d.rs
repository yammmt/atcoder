use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut h: usize,
    }

    let mut ans = 0u64;
    let mut cur = 1u64;
    while h > 0 {
        ans += cur;
        cur *= 2;
        h /= 2;
    }
    println!("{ans}");
}
