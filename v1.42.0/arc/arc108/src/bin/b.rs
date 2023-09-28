use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

// "9 ffoxfoxox" や "6 fofoxx" を考えると削除済みでも遡って検索する必要がある

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    if n < 3 {
        println!("{}", n);
        return;
    }

    let mut ans = n;
    let mut vdq = VecDeque::new();
    for c in &s {
        if *c == 'f' || *c == 'o' {
            vdq.push_back(*c);
        } else if *c == 'x' {
            if vdq.len() < 2 {
                vdq.clear();
                continue;
            }

            let c2 = vdq.pop_back().unwrap();
            let c1 = vdq.pop_back().unwrap();
            if c1 == 'f' && c2 == 'o' {
                ans -= 3;
            } else {
                vdq.clear();
            }
        } else {
            vdq.clear();
        }
    }

    println!("{}", ans);
}
