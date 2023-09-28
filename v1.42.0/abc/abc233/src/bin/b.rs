use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars,
    }
    let l = l - 1;

    let mut ans = vec![];
    (0..l).for_each(|i| ans.push(s[i]));
    (l..r).rev().for_each(|i| ans.push(s[i]));
    (r..s.len()).for_each(|i| ans.push(s[i]));

    println!("{}", ans.iter().collect::<String>());
}
