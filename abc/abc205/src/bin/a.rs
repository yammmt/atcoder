use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }
    println!("{}", b / 100.0 * a);
}
