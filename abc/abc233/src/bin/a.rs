use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    println!("{}", if x >= y { 0 } else { (y - x + 9) / 10 });
}
