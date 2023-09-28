// C にしてはかなり これが灰？
// "azaa", "aaz"

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    // 前から見て a 以外の文字が出たところから
    // 後から見て a 以外の文字が出たところまでを抽出して
    // それが回分であれば良い
    // 例外: すべて a
    if s.iter().all(|&c| c == 'a') {
        println!("Yes");
        return;
    }

    let mut front_i = 0;
    while s[front_i] == 'a' {
        front_i += 1;
    }
    let mut rear_i = s.len() - 1;
    while s[rear_i] == 'a' {
        rear_i -= 1;
    }

    // 前削除量が後削除量より大きいなら先頭の a を消さねばならず不可
    if front_i > s.len() - 1 - rear_i {
        println!("No");
        return;
    }

    let mut new_s = vec![];
    for i in front_i..rear_i + 1 {
        new_s.push(s[i]);
    }
    // println!("{:?}", new_s);

    for i in 0..new_s.len() / 2 {
        if new_s[i] != new_s[new_s.len() - i - 1] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
