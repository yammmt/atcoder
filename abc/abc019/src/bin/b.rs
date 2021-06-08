// 4.5min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = vec![];
    let mut cnt = 1;
    for (i, c) in s.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if s[i - 1] == *c {
            cnt += 1;
        } else {
            ans.push(s[i - 1].to_string());
            ans.push(cnt.to_string());
            cnt = 1;
        }
    }
    // last
    ans.push(s[s.len() - 1].to_string());
    ans.push(cnt.to_string());

    for a in &ans {
        print!("{}", a);
    }
    println!();
}
