// :fu:

use proconio::input;
use proconio::marker::Chars;

#[allow(clippy::collapsible_if)]
fn main() {
    input! {
        n: usize,
        sn: [Chars; n],
    }

    let mut ba = 0;
    let mut begin_b = 0;
    let mut end_a = 0;
    let mut inner_ab = 0;
    for s in &sn {
        for i in 0..s.len() - 1 {
            if s[i] == 'A' && s[i + 1] == 'B' {
                inner_ab += 1;
            }
        }

        if s[0] == 'B' && s[s.len() - 1] == 'A' {
            ba += 1;
        } else if s[0] == 'B' {
            begin_b += 1;
        } else if s[s.len() - 1] == 'A' {
            end_a += 1;
        }
    }
    // println!("ba: {}", ba);
    // println!("begin_b: {}", begin_b);
    // println!("end_a: {}", end_a);
    // println!("inner_ab: {}", inner_ab);

    if ba == 0 {
        println!("{}", inner_ab + begin_b.min(end_a));
    } else {
        if begin_b == 0 && end_a == 0 {
            // BA だけで AB を作る
            println!("{}", inner_ab + ba - 1);
        } else {
            // xxA の後ろあるいは Bxx の前に AB 全追加
            println!("{}", inner_ab + begin_b.min(end_a) + ba);
        }
    }
}
