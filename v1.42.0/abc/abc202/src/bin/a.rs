use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
        c: u16,
    }
    println!("{}", 7 * 3 - a - b - c);
}
