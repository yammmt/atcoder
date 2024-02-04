use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [isize; n],
    }

    let mut first = 0;
    let mut cur = 0;
    for &a in &an {
        cur += a;
        if cur < 0 {
            first = first.max(-cur);
        }
    }

    let mut ans = first;
    for a in an {
        ans += a;
    }

    println!("{ans}");
}
