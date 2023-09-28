use proconio::input;

fn main() {
    input! {
        a: i16,
        b: i16,
    }

    println!(
        "{}",
        if a.abs() > b.abs() {
            "Bug"
        } else if a.abs() < b.abs() {
            "Ant"
        } else {
            "Draw"
        }
    )
}
