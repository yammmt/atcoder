use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    let d = 998_244_353;

    let asum = (a * (1 + a) / 2) % d;
    let bsum = (b * (1 + b) / 2) % d;
    let csum = (c * (1 + c) / 2) % d;

    println!(
        "{}",
        (((asum * bsum) % d) * csum) % d
    );
}
