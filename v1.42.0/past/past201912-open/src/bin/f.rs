use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut vs = vec![];
    let mut curs = vec![];
    let mut is_big_appeared = false;
    for i in 0..s.len() {
        if (s[i] as u8) < b'a' {
            curs.push(s[i].to_ascii_lowercase());
            match is_big_appeared {
                true => {
                    vs.push(curs.iter().collect::<String>());
                    curs.clear();
                    is_big_appeared = false;
                }
                false => {
                    is_big_appeared = true;
                }
            }
        } else {
            curs.push(s[i]);
        }
    }

    vs.sort();
    for a in 0..vs.len() {
        let cur = vs[a].chars().collect::<Vec<char>>();
        for i in 0..cur.len() {
            if i == 0 || i == cur.len() - 1 {
                print!("{}", cur[i].to_ascii_uppercase());
            } else {
                print!("{}", cur[i]);
            }
        }
    }
    println!();
}
