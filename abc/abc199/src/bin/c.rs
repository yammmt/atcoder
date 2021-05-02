// タプルの数字間違えて時間が溶けたパターン

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        tabq: [(usize, usize, usize); q],
    }

    let mut flipped = false;
    for tab in &tabq {
        if tab.0 == 1 {
            let a = tab.1 - 1;
            let b = tab.2 - 1;
            if flipped {
                let aa = (a + n) % (2 * n);
                let bb = (b + n) % (2 * n);
                s.swap(aa, bb);
            } else {
                s.swap(a, b);
            }
        } else {
            flipped = !flipped;
        }
    }

    if !flipped {
        println!("{}", s.iter().collect::<String>());
    } else {
        let mut ans = vec![];
        for i in 0..n {
            ans.push(s[i + n]);
        }
        for c in s.iter().take(n) {
            ans.push(*c);
        }
        println!("{}", ans.iter().collect::<String>());
    }
}
