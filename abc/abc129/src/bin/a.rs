use proconio::input;

fn main() {
    input! {
        p: u32,
        q: u32,
        r: u32,
    }
    println!(
        "{}",
        (p + q).min((p + r).min(q + r))
    );
}
