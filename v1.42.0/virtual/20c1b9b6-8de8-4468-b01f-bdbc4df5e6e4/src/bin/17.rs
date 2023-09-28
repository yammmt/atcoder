use proconio::input;

fn main() {
    input! {
        s: String,
    }
    println!("{}", if &s == "ABC" { "ARC" } else { "ABC" });
}
