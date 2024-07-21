use proconio::fastout;
use proconio::input;
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [i32; n],
    }

    let mut a_yesterday = an[0];
    for &a in an.iter().skip(1) {
        let diff = (a - a_yesterday).abs();
        match a.cmp(&a_yesterday) {
            Ordering::Less => println!("down {diff}"),
            Ordering::Equal => println!("stay"),
            Ordering::Greater => println!("up {diff}"),
        }
        a_yesterday = a;
    }
}
