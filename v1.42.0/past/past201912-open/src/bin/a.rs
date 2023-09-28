use proconio::input;

fn main() {
    input! {
        s: String,
    }
    match s.parse::<u16>() {
        Ok(a) => println!("{}", a * 2),
        Err(_) => println!("error"),
    }
}
