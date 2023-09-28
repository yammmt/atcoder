// 8min

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_plus = s.split('+').collect::<Vec<&str>>();

    let mut ans = 0;
    for a in s_plus {
        if a.len() == 1 && a != "0" {
            ans += 1;
        } else {
            let mut includes_zero = false;
            for c in a.split('*').collect::<Vec<&str>>() {
                if c == "0" {
                    includes_zero = true;
                    break;
                }
            }
            if !includes_zero {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
