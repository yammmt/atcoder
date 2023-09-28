// 4.5min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut g_num = 0;
    let mut p_num = 0;
    let mut win = 0;
    let mut lose = 0;
    for c in &s {
        if p_num == g_num {
            // only g
            if *c == 'p' {
                lose += 1;
            }
            g_num += 1;
        } else if *c == 'g' {
            win += 1;
            p_num += 1;
        } else {
            p_num += 1;
        }
    }

    println!("{}", win - lose);
}
