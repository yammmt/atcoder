use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let replaced_words = [
        ['a', 'x', 'a'],
        ['i', 'x', 'i'],
        ['u', 'x', 'u'],
        ['e', 'x', 'e'],
        ['o', 'x', 'o'],
    ];

    let mut st = VecDeque::new();
    for c in s {
        // もっと弾けるがコードが長くなるので定数倍妥協
        if st.len() < 2 {
            st.push_back(c);
            continue;
        }

        let c1 = st.pop_back().unwrap();
        let c2 = st.pop_back().unwrap();
        let ss = [c2, c1, c];
        let mut replaced = false;
        for w in &replaced_words {
            if ss == *w {
                for _ in 0..3 {
                    st.push_back('.');
                }
                replaced = true;
                break;
            }
        }

        if !replaced {
            st.push_back(c2);
            st.push_back(c1);
            st.push_back(c);
        }
    }

    println!("{}", st.iter().collect::<String>());
}
