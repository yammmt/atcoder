use proconio::input;

fn main() {
    input! {
        n: i128,
    }

    let upper = 2i128.pow(31);
    let lower = -upper;
    println!(
        "{}",
        if lower <= n && n < upper {
            "Yes"
        } else {
            "No"
        }
    );
}
