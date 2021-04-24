use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    println!(
        "{}",
        if a * a + b * b < c * c {
            "Yes"
        } else {
            "No"
        }
    );
}
