use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    }
    let snk_max = a + b * (n - 1);
    let snk_min = b + a * (n - 1);
    println!("{}", (snk_max - snk_min + 1).max(0));
}
