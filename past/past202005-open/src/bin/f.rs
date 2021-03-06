use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        ann: [Chars; n],
    }

    let mut cset = vec![HashSet::new(); n];
    for (i, an) in ann.iter().enumerate() {
        for a in an {
            cset[i].insert(*a);
        }
    }

    let mut ans_front = vec![];
    let mut ans_back = vec![];
    // 前から i 文字目を決める
    for i in 0..n {
        // must be i <= n - i - 1
        if 2 * i + 1 > n {
            break;
        }

        let mut pass = false;
        let i_back = n - i - 1;
        for k_f in &cset[i] {
            if cset[i_back].contains(k_f) {
                ans_front.push(*k_f);
                if i != i_back {
                    ans_back.push(*k_f);
                }
                pass = true;
                break;
            }
        }

        if !pass {
            println!("-1");
            return;
        }
    }

    ans_back.reverse();
    ans_front.append(&mut ans_back);
    println!("{}", ans_front.iter().collect::<String>());
}
