// 入力 `s` の範囲が未定義: 半角大文字、数字、全角文字その他が含まれていないとは言っていないが
// そのような入力はないらしい ただの説明不足っぽい

// WA: ループの break 位置と出力の形式

use std::io;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    input! {
        // s: String,
        n: usize,
        tn: [Chars; n],
    }

    let vs = s.split(' ').collect::<Vec<&str>>();
    let mut ans = vec![];
    for cs in &vs {
        let vc = cs.chars().collect::<Vec<char>>();
        let mut filtered = false;
        'tcheck: for t in &tn {
            if t.len() != vc.len() {
                continue;
            }

            for i in 0..vc.len() {
                if t[i] != '*' && t[i] != vc[i] {
                    break;
                }

                if i == vc.len() - 1 {
                    filtered = true;
                    ans.push(std::iter::repeat('*').take(vc.len()).collect::<String>());
                    break 'tcheck;
                }
            }
        }
        if !filtered {
            ans.push(vc.iter().collect::<String>());
        }
    }

    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
