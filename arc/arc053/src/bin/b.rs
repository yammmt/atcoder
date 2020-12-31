// :fu:

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt = vec![0; 26];
    for c in &s {
        cnt[(*c as u8 - b'a') as usize] += 1;
    }
    cnt.sort_unstable();
    cnt.reverse();
    let mut evens = vec![];
    let mut odds = vec![];
    for c in &cnt {
        if *c == 0 {
            break;
        }

        if *c % 2 == 0 {
            evens.push(*c);
        } else {
            odds.push(*c);
        }
    }
    // println!("e: {:?}", evens);
    // println!("o: {:?}", odds);

    if odds.is_empty() {
        println!("{}", evens.iter().sum::<u32>());
        return;
    }

    // (元の文字列の長さ - 奇数回登場した文字の数) 数の文字でペア作って分配
    println!(
        "{}",
        ((s.len() - odds.len()) / 2 / odds.len()) * 2 + 1
    );
}
