// 体感 AB より明らかに楽だったが

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }
    let mut ans = 0;
    let mut charcnt = vec![0; 26];

    // 後ろから見ていって以後全部置き換える
    for i in (0..s.len()).rev() {

        charcnt[(s[i] as u8 - b'a') as usize] += 1;
        if i + 2 >= s.len() {
            continue;
        }

        if s[i] == s[i + 1] && s[i] != s[i + 2] {
            ans += s.len() - i - charcnt[(s[i] as u8 - b'a') as usize];
            s[i + 2] = s[i];
            charcnt = vec![0; 26];
            charcnt[(s[i] as u8 - b'a') as usize] = s.len() - i;
        }
    }

    println!("{}", ans);
}
