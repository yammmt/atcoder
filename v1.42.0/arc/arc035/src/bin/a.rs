// 3.5min

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut could_kaibun = true;
    for i in 0..s.len() {
        let j = s.len() - i - 1;
        if s[i] != '*' && s[j] != '*' && s[i] != s[j] {
            could_kaibun = false;
            break;
        }

        if i == j || i + 1 == j {
            break;
        }
    }

    println!(
        "{}",
        match could_kaibun {
            true => "YES",
            false => "NO",
        }
    );
}
