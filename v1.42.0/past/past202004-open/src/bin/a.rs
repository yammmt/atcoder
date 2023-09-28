// 脳筋解法

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let floors = [
        "B9",
        "B8",
        "B7",
        "B6",
        "B5",
        "B4",
        "B3",
        "B2",
        "B1",
        "1F",
        "2F",
        "3F",
        "4F",
        "5F",
        "6F",
        "7F",
        "8F",
        "9F"
    ];
    let mut hm = HashMap::new();
    for i in 0..18 {
        hm.insert(floors[i].to_string(), i as i32);
    }

    println!(
        "{}",
        (hm.get(&s).unwrap() - hm.get(&t).unwrap()).abs()
    );
}
