// :fu: :fu: 21-05 シンプルに嫌いな類の数問

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let g = gcd(a, b);
    if a / g > 10u64.pow(18) / b {
        println!("Large");
    } else {
        println!("{}", a / g * b);
    }
}
