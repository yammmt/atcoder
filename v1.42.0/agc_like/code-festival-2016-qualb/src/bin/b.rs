// 5min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }
    let mut a_num = 0;
    let mut b_num = 0;
    for c in &s {
        // println!("{} {}", a_num, b_num);
        println!(
            "{}",
            match *c {
                'a' => {
                    if a_num + b_num < a + b {
                        a_num += 1;
                        "Yes"
                    } else {
                        "No"
                    }
                }
                'b' => {
                    if a_num + b_num < a + b && b_num < b {
                        b_num += 1;
                        "Yes"
                    } else {
                        "No"
                    }
                }
                'c' => "No",
                _ => unreachable!(),
            }
        );
    }
}
