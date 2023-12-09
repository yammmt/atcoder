use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        _m: usize,
        s: Chars,
        t: Chars,
    }

    let mut is_prefix = true;
    for i in 0..s.len() {
        if s[i] != t[i] {
            is_prefix = false;
        }
    }

    let mut is_suffix = true;
    let mut ss = s;
    ss.reverse();
    let mut tt = t;
    tt.reverse();
    for i in 0..ss.len() {
        if ss[i] != tt[i] {
            is_suffix = false;
        }
    }

    println!(
        "{}",
        if is_prefix && is_suffix {
            0
        } else if is_prefix && !is_suffix {
            1
        } else if !is_prefix && is_suffix {
            2
        } else {
            3
        }
    );
}
