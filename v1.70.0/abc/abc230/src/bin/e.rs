// D より圧倒的に難しくないか

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    let mut i = 1;
    while i * i <= n {
        ans += n / i;
        i += 1;
    }
    ans *= 2;
    if i * i > n {
        i -= 1;
    }
    ans -= i * i;

    println!("{ans}");
}
