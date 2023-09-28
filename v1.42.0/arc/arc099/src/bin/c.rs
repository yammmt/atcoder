use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        _an: [u64; n],
    }
    println!(
        "{}",
        ((n - 1) + (k - 1) - 1) / (k - 1)
    );
}
