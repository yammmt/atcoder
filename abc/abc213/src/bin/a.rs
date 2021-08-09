use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    }
    for i in 0..=255 {
        if a ^ i == b {
            println!("{}", i);
            return;
        }
    }
}
