use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut hn: [usize; n],
    }
    hn.sort_unstable();
    hn.reverse();
    let ans = hn.iter().skip(k).sum::<usize>();
    println!("{ans}");
}
