// 16min 1WA
// WA: 最多出現の文字のみを試していた

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = std::u64::MAX;
    for ci in 0..26 {
        let mut curs = s.clone();
        let mut curp = 0;
        while !curs.iter().all(|&c| c as u8 - b'a' == ci) {
            let mut nexts = vec![];
            for i in 0..curs.len() - 1 {
                nexts.push(
                    if curs[i] as u8 - b'a' == ci {
                        curs[i]
                    } else {
                        curs[i + 1]
                    }
                );
            }
            curs = nexts;
            curp += 1;
        }
        ans = ans.min(curp);
    }
    println!("{}", ans);
}
