use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: i64,
        r: i64,
        n: i64,
    }

    let mut ans = a;
    for _ in 0..n - 1 {
        ans *= r;
        if ans > 1_000_000_000 {
            println!("large");
            return;
        }
    }

    println!("{ans}");
}
