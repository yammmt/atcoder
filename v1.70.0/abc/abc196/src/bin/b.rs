use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
    }

    let mut ans = vec![];
    for c in &x {
        if *c == '.' {
            break;
        }

        ans.push(*c);
    }

    println!("{}", ans.iter().collect::<String>());
}
