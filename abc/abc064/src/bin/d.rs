// 27min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
    }

    let mut stored_left_half = 0;
    let mut left_half_added = 0;
    for c in &s {
        match *c {
            '(' => {
                stored_left_half += 1;
            }
            ')' => match stored_left_half {
                0 => {
                    left_half_added += 1;
                }
                _ => {
                    stored_left_half -= 1;
                }
            },
            _ => unreachable!(),
        }
    }
    let right_half_added = stored_left_half;

    let mut ans = vec![];
    ans.append(&mut vec!['('; left_half_added]);
    ans.append(&mut s);
    ans.append(&mut vec![')'; right_half_added]);

    println!("{}", ans.iter().collect::<String>());
}
