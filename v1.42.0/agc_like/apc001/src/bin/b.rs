// :fu:

use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
        bn: [i64; n],
    }

    let asum = an.iter().sum::<i64>();
    let bsum = bn.iter().sum::<i64>();
    if asum > bsum {
        println!("No");
        return;
    }
    let sosa = bsum - asum;
    // println!("{}", sosa);

    let mut required_plus = 0;
    let mut required_minus = 0;
    for i in 0..n {
        match an[i].cmp(&bn[i]) {
            // 端数切り上げ
            Ordering::Less => required_plus += (bn[i] - an[i] + 2 - 1) / 2,
            Ordering::Greater => required_minus += an[i] - bn[i],
            _ => {}
        }
    }
    // println!("{}", required_plus);
    // println!("{}", required_minus);

    if required_plus > sosa || required_minus > sosa {
        println!("No");
    } else {
        println!("Yes");
    }
}
