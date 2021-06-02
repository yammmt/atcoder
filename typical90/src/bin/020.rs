use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    println!("{}", if a < c.pow(b as u32) { "Yes" } else { "No" });
}
