use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if s == "Sunny" {
        println!("Cloudy");
    } else if s == "Cloudy" {
        println!("Rainy");
    } else {
        println!("Sunny");
    }
}
