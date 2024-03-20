use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
        q: usize,
        cdq: [(char, char); q],
    }

    let mut char_map = vec![];
    for i in 0..26 {
        char_map.push((b'a' + i) as char);
    }

    for (c, d) in cdq {
        for i in 0..26 {
            if char_map[i] == c {
                char_map[i] = d;
            }
        }
    }

    let mut ans = vec![];
    for c in s {
        let ci = (c as u8 - b'a') as usize;
        ans.push(char_map[ci]);
    }

    println!("{}", ans.iter().collect::<String>());
}
