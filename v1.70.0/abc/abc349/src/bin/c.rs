use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut t_i = 0;
    for c in s {
        if c.to_ascii_uppercase() == t[t_i] {
            t_i += 1;
            if t_i > 2 {
                break;
            }
        }
    }

    println!(
        "{}",
        if t_i > 2 || (t_i == 2 && *t.last().unwrap() == 'X') {
            "Yes"
        } else {
            "No"
        }
    );
}
