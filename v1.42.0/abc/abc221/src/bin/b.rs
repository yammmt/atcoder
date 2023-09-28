use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut difference = vec![];
    for i in 0..s.len() {
        if s[i] != t[i] {
            difference.push(i);
        }
    }

    if difference.is_empty()
        || (difference.len() == 2
            && difference[1] - difference[0] == 1
            && s[difference[0]] == t[difference[1]]
            && s[difference[1]] == t[difference[0]])
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
