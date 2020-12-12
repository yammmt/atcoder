use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut a_idx = 0;
    let mut a_idx_found = false;
    for i in 0..s.len() {
        match a_idx_found {
            true => {
                if s[i] != 'Z' {
                    continue;
                }

                ans = ans.max(i - a_idx + 1);
            },
            false => {
                if s[i] != 'A' {
                    continue;
                }

                a_idx = i;
                a_idx_found = true;
            }
        }
    }
    println!("{}", ans);
}
