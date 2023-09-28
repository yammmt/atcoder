// 3min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut current = vec![];
    for c in &s {
        match *c {
            '0' => current.push('0'),
            '1' => current.push('1'),
            'B' => {
                current.pop();
            }
            _ => unreachable!(),
        }
    }
    println!("{}", current.iter().collect::<String>());
}
