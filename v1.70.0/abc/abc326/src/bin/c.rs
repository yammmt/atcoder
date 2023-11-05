use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut an: [usize; n],
    }
    an.sort_unstable();

    // 始点固定でしゃくとり
    let mut ans = 0;
    let mut r = 0;
    for l in 0..n {
        while r < n && an[r] - an[l] < m {
            r += 1;
        }
        ans = ans.max(r - l);
    }

    println!("{ans}");
}
