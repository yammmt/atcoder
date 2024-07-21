use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    match s.parse::<usize>() {
        Ok(n) => println!("{}", n * 2),
        Err(_) => println!("error"),
    }
}
