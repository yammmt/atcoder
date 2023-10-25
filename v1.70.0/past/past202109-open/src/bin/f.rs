use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // '0' 要素を一つずつずらしてやればよいだけでは
    let mut v = vec![];
    for (i, c) in s.iter().enumerate() {
        if *c == '0' {
            v.push(i);
        }
    }

    if v.len() == 1 {
        println!("-1");
        return;
    }

    let mut j = 0;
    for i in 0..n {
        if j < v.len() && i == v[j] {
            print!("{}", v[(j + 1) % v.len()] + 1);
            j += 1;
        } else {
            print!("{}", i + 1);
        }

        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
