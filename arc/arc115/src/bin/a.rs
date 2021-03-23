// :fu: 21-03

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        _m: usize,
        sn: [Chars; n],
    }

    // 愚直には異なる解答をした部分が奇数個である組の数だが TLE
    let mut zero_e = 0i64;
    let mut zero_o = 0i64;
    for s in &sn {
        if s.iter().filter(|&c| *c == '0').count() % 2 == 0 {
            zero_e += 1;
        } else {
            zero_o += 1;
        }
    }
    println!("{}", zero_o * zero_e);
}
