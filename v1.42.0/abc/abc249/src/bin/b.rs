use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }

    let mut hs = HashSet::new();
    let mut big_appears = false;
    let mut small_appears = false;
    for c in &s {
        if hs.contains(c) {
            println!("No");
            return;
        }

        hs.insert(c);
        if c.is_ascii_uppercase() {
            big_appears = true;
        } else {
            small_appears = true;
        }
    }

    println!(
        "{}",
        if big_appears && small_appears {
            "Yes"
        } else {
            "No"
        }
    );
}
