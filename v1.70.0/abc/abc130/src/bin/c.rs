use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        w: f64,
        h: f64,
        x: f64,
        y: f64,
    }

    let area = w * h / 2.0;
    println!(
        "{area} {}",
        if !(x == w / 2.0 && y == h / 2.0) {
            0
        } else {
            1
        }
    );
}
