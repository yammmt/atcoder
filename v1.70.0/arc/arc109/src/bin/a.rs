use proconio::fastout;
use proconio::input;
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
        y: usize,
    }

    // なにも考えずに Dijkstra でも通りそう

    match a.cmp(&b) {
        Ordering::Less => {
            let ba = b - a;
            println!("{}", (x + ba * y).min((2 * ba + 1) * x));
        }
        Ordering::Equal => {
            println!("{x}");
        }
        Ordering::Greater => {
            // 嘘っぽさがある
            // x でできるだけ進んで最後に y はない？
            let ab = a - b;
            println!("{}", ((2 * ab - 1) * x).min(x + (ab - 1) * y));
        }
    }
}
