use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
        c: u16,
    }
    if (a <= c && c <= b) || (b <= c && c <= a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
