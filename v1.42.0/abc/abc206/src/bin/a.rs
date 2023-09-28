use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: u16,
    }
    let real = n * 108 / 100;
    println!(
        "{}",
        match real.cmp(&206) {
            Ordering::Less => "Yay!",
            Ordering::Equal => "so-so",
            Ordering::Greater => ":(",
        }
    );
}
