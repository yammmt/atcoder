// :fu:

use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut fibo = vec![0u64; 43];
    fibo[1] = 1;
    fibo[2] = 1;
    for i in 3..fibo.len() {
        fibo[i] = fibo[i - 1] + fibo[i - 2];
    }
    // println!("{:?}", fibo);

    println!(
        "{} {}",
        fibo[k + 2], fibo[k + 1]
    );
}
