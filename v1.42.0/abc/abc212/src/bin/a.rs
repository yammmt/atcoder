use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    println!(
        "{}",
        if a > 0 && b == 0 {
            "Gold"
        } else if a == 0 && b > 0 {
            "Silver"
        } else {
            "Alloy"
        }
    );
}
