use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ptrn = vec!['o', 'x', 'x'];
    for begin_i in 0..3 {
        let mut pass = true;
        for j in 0..s.len() {
            if s[j] != ptrn[(begin_i + j) % 3] {
                pass = false;
                break;
            }
        }

        if pass {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
