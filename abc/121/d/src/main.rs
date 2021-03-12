// :fu: :fu: 21-03

use proconio::input;

fn xor_sum(n: u64) -> u64 {
    let one_num = (n + 1) / 2;
    match n % 2 {
        0 => (one_num % 2) ^ n,
        _ => one_num % 2,
    }
}

fn main() {
    input! {
        a: u64,
        b: u64,
    }
    println!(
        "{}",
        if a == 0 {
            xor_sum(b)
        } else {
            xor_sum(b) ^ xor_sum(a - 1)
        }
    );
}
