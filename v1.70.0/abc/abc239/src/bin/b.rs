use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    println!(
        "{}",
        if x >= 0 {
            x / 10
        } else {
            if x % 10 == 0 {
                x / 10
            } else {
                x / 10 - 1
            }
        }
    );
}
