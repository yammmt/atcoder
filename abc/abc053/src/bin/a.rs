use proconio::input;

fn main() {
    input! {
        x: u16,
    }
    println!(
        "{}",
        if x < 1200 {
            "ABC"
        } else {
            "ARC"
        }
    );
}
