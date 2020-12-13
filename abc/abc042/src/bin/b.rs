// 1.5min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        _l: usize,
        sn: [Chars; n],
    }

    let mut strs = vec![];
    for s in &sn {
        strs.push(s.iter().collect::<String>());
    }
    strs.sort();
    for s in &strs {
        print!("{}", s);
    }
    println!();
}
