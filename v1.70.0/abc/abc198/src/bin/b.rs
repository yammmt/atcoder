use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut n: Chars,
    }

    while n.last() == Some(&'0') {
        n.pop();
    }

    for i in 0..n.len() / 2 {
        if n[i] != n[n.len() - i - 1] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
