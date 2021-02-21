use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }
    let mut ans = 0;
    // let mut lastpos = [[s.len() - 1, s.len() - 1]; 26];
    let mut charcnt = vec![0; 26];

    // 後ろから見ていって以後全部置き換える
    for i in (0..s.len()).rev() {
        // println!("i: {}/ s[i]: {}", i, s[i]);
        // println!("{:?}", lastpos);

        charcnt[(s[i] as u8 - b'a') as usize] += 1;
        if i + 2 >= s.len() {
            // lastpos[(s[i] as u8 - b'a') as usize][1] = lastpos[(s[i] as u8 - b'a') as usize][0];
            // lastpos[(s[i] as u8 - b'a') as usize][0] = i;
            continue;
        }

        if s[i] == s[i + 1] && s[i] != s[i + 2] {
            // let prevpos = lastpos[(s[i] as u8 - b'a') as usize][1];
            // ans += prevpos - i - 1 - charcnt[(s[i] as u8 - b'a') as usize];
            ans += s.len() - i - charcnt[(s[i] as u8 - b'a') as usize];
            s[i + 2] = s[i];
            // lastpos = [[s.len() - 1, s.len() - 1]; 26];
            // lastpos[(s[i] as u8 - b'a') as usize][0] = i;
            charcnt = vec![0; 26];
            charcnt[(s[i] as u8 - b'a') as usize] = s.len() - i;
            // println!("  ans: {}", ans);
        } else {
            // lastpos[(s[i] as u8 - b'a') as usize][1] = lastpos[(s[i] as u8 - b'a') as usize][0];
            // lastpos[(s[i] as u8 - b'a') as usize][0] = i;
        }
    }

    println!("{}", ans);
}
