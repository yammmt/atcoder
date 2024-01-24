// めちゃくちゃ苦手

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: Chars,
        sn: [Chars; n],
    }

    let mut ans = vec![];

    let is_good = |s_short: &[char], s_long: &[char]| {
        // println!("{:?}", s_short);
        // println!("{:?}", s_long);
        let mut i = 0;
        let mut j = 0;
        let mut diff_appeared = false;
        while i < s_short.len() && j < s_long.len() {
            // println!("{i} {j}");
            if s_short[i] == s_long[j] {
                i += 1;
                j += 1;
            } else {
                // println!("  {} != {}", s_short[i], s_long[j]);
                if diff_appeared {
                    return false;
                }

                diff_appeared = true;
                j += 1;
                if s_short.len() == s_long.len() {
                    i += 1;
                }
            }
        }

        true
    };

    for (i, s) in sn.iter().enumerate() {
        if (s.len() == t.len() && is_good(s, &t))
            || (s.len() - 1 == t.len() && is_good(&t, s))
            || (t.len() - 1 == s.len() && is_good(s, &t))
        {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.len());
    for (i, a) in ans.iter().enumerate() {
        print!("{a}");
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
