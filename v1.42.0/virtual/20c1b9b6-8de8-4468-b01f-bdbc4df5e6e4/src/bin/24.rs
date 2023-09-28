use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    for i in 0..3 {
        let mut pass = true;
        for j in 0..s.len() {
            if (j % 3 == i && s[j] == 'x') || (j % 3 != i && s[j] == 'o') {
                pass = false;
            }
        }

        if pass {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
