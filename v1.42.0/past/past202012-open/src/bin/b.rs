use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut t = vec![];
    for c in &s {
        let mut new_t = vec![];
        t.iter()
            .filter(|&tc| tc != c)
            .for_each(|tt| new_t.push(*tt));
        new_t.push(*c);
        t = new_t;
    }

    println!("{}", t.iter().collect::<String>());
}
