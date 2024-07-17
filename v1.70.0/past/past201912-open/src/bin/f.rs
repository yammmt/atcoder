use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut vs = vec![];
    let mut cur = vec![];
    for c in s {
        if c.is_ascii_uppercase() {
            cur.push(c);
            if cur.len() == 1 {
                continue;
            }

            vs.push(cur.iter().collect::<String>());
            cur.clear();
        } else {
            cur.push(c);
        }
    }

    vs.sort_unstable_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));

    for (i, s) in vs.iter().enumerate() {
        print!("{s}");
        if i == vs.len() - 1 {
            println!();
        }
    }
}
