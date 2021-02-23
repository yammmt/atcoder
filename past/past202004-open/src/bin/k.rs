// :fu: 21-02 解けたい

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        cn: [i64; n],
        dn: [i64; n],
    }

    let mut l_num = 0;
    let mut r_num = 0;
    for c in &s {
        if *c == '(' {
            l_num += 1;
        } else if *c == ')' {
            r_num += 1;
        }
    }

    let mut ans = 0;

    println!("{}", ans);
}
