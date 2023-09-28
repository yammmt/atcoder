use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
    }
    println!(
        "{}",
        if (s1[0] == '#' && s1[1] == '.' && s2[0] == '.' && s2[1] == '#')
            || (s1[0] == '.' && s1[1] == '#' && s2[0] == '#' && s2[1] == '.')
        {
            "No"
        } else {
            "Yes"
        }
    );
}
