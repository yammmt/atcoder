use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    println!("{}", if s < t { "Yes" } else { "No" });
}
