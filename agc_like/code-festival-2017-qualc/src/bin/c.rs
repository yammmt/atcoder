// > 90 min
// 配列操作で無限にバグ

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    if s.len() == 1 {
        println!("0");
        return;
    }

    let mut il = 0;
    let mut ir = s.len() - 1;
    let mut ans = 0;
    loop {
        if s[il] == 'x' && s[ir] != 'x' {
            ans += 1;
            il += 1;
        } else if s[il] != 'x' && s[ir] == 'x' {
            ans += 1;
            ir -= 1;
        } else if s[il] != s[ir] {
            println!("-1");
            return;
        } else {
            il += 1;
            ir -= 1;
        }

        if il >= ir {
            break;
        }
    }
    println!("{}", ans);
}
