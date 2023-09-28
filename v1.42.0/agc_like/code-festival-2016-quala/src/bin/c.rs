// 13min
// WA: 'a' を 'a' に変換していた

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        k: usize,
    }

    let mut process_left = k;
    // 先頭からできるだけ a にする
    for c in &mut s {
        if *c == 'a' {
            continue;
        }

        let required_k = 26 - (*c as u8 - b'a');
        if process_left >= required_k as usize {
            *c = 'a';
            process_left -= required_k as usize;
        }
    }
    // 後ろで帳尻をあわせる
    let slen = s.len() - 1;
    s[slen] = (((s[slen] as usize - b'a' as usize + process_left) % 26) + b'a' as usize) as u8 as char;

    println!("{}", s.iter().collect::<String>());
}
