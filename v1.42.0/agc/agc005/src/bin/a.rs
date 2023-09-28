// 4min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
    }

    let mut deleted = 0;
    let mut stored_s = 0;
    for c in &x {
        match *c {
            'S' => stored_s += 1,
            'T' => {
                if stored_s > 0 {
                    deleted += 2;
                    stored_s -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", x.len() - deleted);
}
