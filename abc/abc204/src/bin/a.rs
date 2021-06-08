use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    println!("{}", if x == y { x } else { 3 - x - y });
}
