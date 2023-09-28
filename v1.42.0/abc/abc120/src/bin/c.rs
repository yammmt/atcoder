// 6min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let zero_num = s.iter().filter(|&c| *c == '0').count();
    let one_num = s.iter().filter(|&c| *c == '1').count();
    println!("{}", zero_num.min(one_num) * 2);
}
