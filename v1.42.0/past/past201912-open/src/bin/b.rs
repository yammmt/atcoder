use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }
    for i in 1..n {
        println!(
            "{}",
            match an[i].cmp(&an[i - 1]) {
                Ordering::Less => format!("down {}", an[i - 1] - an[i]),
                Ordering::Equal => String::from("stay"),
                Ordering::Greater => format!("up {}", an[i] - an[i - 1]),
            }
        );
    }
}
