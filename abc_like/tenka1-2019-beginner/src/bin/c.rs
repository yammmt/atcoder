use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // '.': white, '#': black
    let mut left_black = 0;
    let mut right_white = s.iter().filter(|&c| *c == '.').count();

    let mut ans = n + 1;
    // i 文字目より左をすべて白に, i 文字目以降をすべて黒に
    for (i, c) in s.iter().enumerate() {
        ans = ans.min(left_black + right_white);
        if s[i] == '#' {
            left_black += 1;
        }
        if s[i] == '.' {
            right_white -= 1;
        }
    }
    ans = ans.min(left_black + right_white);

    println!("{}", ans);
}
