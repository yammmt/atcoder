use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut cur = 0;
    for c in s {
        match c {
            'A' | 'C' | 'G' | 'T' => cur += 1,
            _ => cur = 0,
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
