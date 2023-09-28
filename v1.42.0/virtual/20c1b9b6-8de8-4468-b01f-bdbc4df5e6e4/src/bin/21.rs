// 嫌

use proconio::input;
use proconio::marker::{Bytes, Chars};

fn main() {
    input! {
        x: Bytes,
        n: usize,
        sn: [Chars; n],
    }

    // 新しいアルファベット順で何番目か
    let mut new_pos = vec![0; 26];
    for (i, c) in x.iter().enumerate() {
        new_pos[(*c - b'a') as usize] = i;
    }

    let mut new_strings = vec![];
    for (i, s) in sn.iter().enumerate() {
        let mut vc = vec![];
        for c in s {
            vc.push(new_pos[(*c as u8 - b'a') as usize]);
        }
        new_strings.push((vc, i));
    }
    new_strings.sort_unstable();

    for ns in new_strings {
        println!("{}", sn[ns.1].iter().collect::<String>());
    }
}
