use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s3: [String; 3],
    }
    let mut hs = HashSet::new();
    s3.iter().for_each(|s| { hs.insert(s.as_str()); });
    println!(
        "{}",
        if !hs.contains("ABC") {
            "ABC"
        } else if !hs.contains("ARC") {
            "ARC"
        } else if !hs.contains("AGC") {
            "AGC"
        } else {
            "AHC"
        }
    );
}
