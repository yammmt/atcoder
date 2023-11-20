use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let day = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    for (i, d) in day.iter().enumerate() {
        if *d == s {
            println!("{}", 7 - i);
            return;
        }
    }
}
