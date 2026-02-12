use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        mut an: [usize; n],
    }
    an.push(t);

    let mut ans = 0;
    let mut last = 0;
    for a in an {
        if last > a {
            continue;
        }

        ans += a - last;
        last = a + 100;
    }

    println!("{ans}");
}
