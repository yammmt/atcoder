use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    println!("{}", if a <= b && b <= 6 * a { "Yes" } else { "No" });
}
