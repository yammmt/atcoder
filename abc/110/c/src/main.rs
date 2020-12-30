// -*- coding:utf-8-unix -*-

// 嘘解法を通したままにしない
// 嘘解法: "aab" と "cdd" で Yes を返してはならない
// after_contests は追加されていない

// WA: 定義済の変換表を上書きしない

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    // s と t の長さは入力の時点で等しい
    let vs = s.chars().collect::<Vec<char>>();
    let vt = t.chars().collect::<Vec<char>>();
    let mut s2t = vec![0; 26];
    let mut convert_to_defined = vec![false; 26];
    for i in 0..vs.len() {
        let vs_idx = (vs[i] as u8 - b'a') as usize;
        let vt_idx = (vt[i] as u8 - b'a') as usize;

        if s2t[vs_idx] != vt[i] as u8 {
            if s2t[vs_idx] == 0 && !convert_to_defined[vt_idx] {
                s2t[vs_idx] = vt[i] as u8;
                convert_to_defined[vt_idx] = true;
            } else {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
