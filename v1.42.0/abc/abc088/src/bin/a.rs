use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
    }
    println!(
        "{}",
        if n % 500 <= a {
            "Yes"
        } else {
            "No"
        }
    );
}
