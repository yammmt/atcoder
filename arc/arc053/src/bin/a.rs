use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    println!(
        "{}",
        (w - 1) * h + (h - 1) * w
    );
}
