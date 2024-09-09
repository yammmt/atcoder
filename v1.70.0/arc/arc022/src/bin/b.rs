use proconio::fastout;
use proconio::input;

const A_MAX: usize = 100_001;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut ans = 0;
    let mut appeared = vec![false; A_MAX];
    let mut r = 0;
    for l in 0..n {
        if r < l {
            r = l;
        }

        while r < n && !appeared[an[r]] {
            appeared[an[r]] = true;
            r += 1;
        }

        ans = ans.max(r - l);
        appeared[an[l]] = false;
    }

    println!("{ans}");
}
