use proconio::input;

fn main() {
    input! {
        s: char,
        t: String,
    }
    println!("{}", if s == 'Y' { t.to_uppercase() } else { t });
}
