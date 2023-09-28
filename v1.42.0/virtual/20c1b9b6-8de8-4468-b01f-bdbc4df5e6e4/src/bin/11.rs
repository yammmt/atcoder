use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut cubes = vec![];
    for c in &s {
        if cubes.is_empty() || cubes.last().unwrap() == &c {
            cubes.push(c);
        } else {
            cubes.pop();
        }
    }

    println!("{}", s.len() - cubes.len());
}
