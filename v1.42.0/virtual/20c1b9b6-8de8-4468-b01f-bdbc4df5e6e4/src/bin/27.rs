use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }
    println!(
        "{}",
        match (a + b).cmp(&(c + d)) {
            Ordering::Greater => "Left",
            Ordering::Equal => "Balanced",
            Ordering::Less => "Right",
        }
    );
}
