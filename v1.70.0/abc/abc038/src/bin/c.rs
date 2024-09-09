use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut ans = 0;
    let mut r = 0;
    for l in 0..n {
        if r < l + 1 {
            r = l + 1;
        }

        while r < n && an[r] > an[r - 1] {
            r += 1;
        }

        ans += r - l;
    }

    println!("{ans}");
}
