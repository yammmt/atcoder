use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut a: [usize; 6],
    }
    a.sort_unstable();
    a.reverse();
    println!("{}", a[2]);
}
