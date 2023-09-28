// 嘘解法を通したままにしない
// 嘘解法: "aab" と "cdd" で Yes を返してはならない
// after_contests は追加されていない

// WA: 定義済の変換表を上書きしない

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s2t = vec![None; 26];
    let mut s_exists = vec![false; 26];
    for i in 0..s.len() {
        let cur_s = (s[i] as u8 - b'a') as usize;
        let cur_t = (t[i] as u8 - b'a') as usize;

        // println!("{}", s[i]);
        if let Some(c) = s2t[cur_s] {
            if c != cur_t {
                println!("No");
                return;
            }
        } else {
            if s_exists[cur_t] {
                println!("No");
                return;
            }

            s2t[cur_s] = Some(cur_t);
            s_exists[cur_t] = true;
        }
    }

    println!("Yes");
}
