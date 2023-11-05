use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    // omg
    let sl = s.iter().collect::<String>().to_lowercase();
    let tl = t.iter().collect::<String>().to_lowercase();

    println!(
        "{}",
        if s == t {
            "same"
        } else if sl == tl {
            "case-insensitive"
        } else {
            "different"
        }
    );
}
