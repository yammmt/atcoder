use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }
    s.reverse();

    let dreamer = "dreamer".chars().rev().collect::<String>();
    let eraser = "eraser".chars().rev().collect::<String>();
    let dream = "dream".chars().rev().collect::<String>();
    let erase = "erase".chars().rev().collect::<String>();

    let mut i = 0;
    while i < s.len() {
        let cur7 = if i + 6 < s.len() {
            Some(s.iter().skip(i).take(7).collect::<String>())
        } else {
            None
        };

        let cur6 = if i + 5 < s.len() {
            Some(s.iter().skip(i).take(6).collect::<String>())
        } else {
            None
        };

        let cur5 = if i + 4 < s.len() {
            Some(s.iter().skip(i).take(5).collect::<String>())
        } else {
            None
        };

        if cur7 == Some(dreamer.clone()) {
            i += 7;
        } else if cur6 == Some(eraser.clone()) {
            i += 6;
        } else if cur5 == Some(dream.clone()) || cur5 == Some(erase.clone()) {
            i += 5;
        } else {
            println!("NO");
            return;
        }
    }

    println!("YES");
}
