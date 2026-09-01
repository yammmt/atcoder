use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut a: isize,
        mut b: isize,
        c: isize,
    }
    if b + c == a {
        std::mem::swap(&mut a, &mut b);
    }
    if c > 0 {
        println!("{a} -> {b} (+{c})");
    } else {
        println!("{a} -> {b} ({c})");
    }
}
