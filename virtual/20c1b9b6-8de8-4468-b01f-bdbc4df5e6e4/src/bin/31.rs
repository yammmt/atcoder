use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    }
    let mut ans = a - 1;
    if b >= a {
        ans += 1;
    }
    println!("{}", ans);
}
