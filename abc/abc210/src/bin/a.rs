use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        x: i32,
        y: i32,
    }

    println!(
        "{}",
        if n <= a {
            n * x
        } else {
            a * x + (n - a) * y
        }
    );
}
