// 22min 1WA
// WA: 入力が奇数長かつ既に回文であれば真ん中の文字は変更不可

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        a: Chars,
    }

    let mut rev_a = a.clone();
    rev_a.reverse();
    let is_original_kaibun = a.iter().collect::<String>() == rev_a.iter().collect::<String>();

    let mut same_char_idxes = HashSet::new();
    let mut idx_front = 0;
    let mut idx_rear = a.len() - 1;
    while idx_front <= idx_rear {
        if a[idx_front] == a[idx_rear] {
            same_char_idxes.insert(idx_front);
            if idx_front != idx_rear {
                same_char_idxes.insert(idx_rear);
            }
        }
        idx_front += 1;
        if idx_rear > 0 {
            idx_rear -= 1;
        }
    }

    let mut ans = 0u64;
    for i in 0..a.len() {
        if i == a.len() / 2 && a.len() % 2 == 1 {
            // 回文が成り立っていると変えた後でも回文となってしまうので弾く
            if !is_original_kaibun {
                ans += 25;
            }
            continue;
        }

        if same_char_idxes.contains(&i) {
            // ペアが同じ字であり何に変えても良い
            ans += 25;
        } else if same_char_idxes.len() == a.len() - 2 {
            // 一文字だけ違う
            ans += 24;
        } else {
            // どう変えたところで回文にならないので何しても良い
            ans += 25;
        }
    }

    println!("{}", ans);
}
