use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut a_idx = 0;
    let mut a_idx_found = false;
    for (i, c) in s.iter().enumerate() {
        match a_idx_found {
            true => {
                if *c != 'Z' {
                    continue;
                }

                ans = ans.max(i - a_idx + 1);
            },
            false => {
                if *c != 'A' {
                    continue;
                }

                a_idx = i;
                a_idx_found = true;
            }
        }
    }
    println!("{}", ans);
}
