use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let has_lower = s.iter().any(|c| c.is_ascii_lowercase());
    let has_upper = s.iter().any(|c| c.is_ascii_uppercase());
    let kinds: HashSet<&char> = HashSet::from_iter(s.iter());
    println!(
        "{}",
        if has_lower && has_upper && kinds.len() == s.len() {
            "Yes"
        } else {
            "No"
        }
    );
}
