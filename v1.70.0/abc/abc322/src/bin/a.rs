use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut ans: isize = -1;
    for i in 0..s.len() - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            ans = i as isize + 1;
            break;
        }
    }

    println!("{ans}");
}
