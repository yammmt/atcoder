use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut number_num = 0;
    for &c in &s {
        if c >= '0' && c <= '9' {
            number_num += 1;
        }
    }

    println!(
        "{}",
        if number_num == 7 && s[3] == '-' {
            "Yes"
        } else {
            "No"
        }
    );
}
