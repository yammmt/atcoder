use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = vec![];
    for c in s {
        ans.push(match c {
            'a' => 'A',
            'i' => 'I',
            'u' => 'U',
            'e' => 'E',
            'o' => 'O',
            _ => c,
        })
    }
    println!("{}", ans.iter().collect::<String>());
}
