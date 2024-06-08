use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: Chars,
        cn: [Usize1; n],
    }

    let mut colored_chars = vec![vec![]; m];
    for (i, &c) in cn.iter().enumerate() {
        colored_chars[c].push(i);
    }

    for i in 0..m {
        let mut written = s[*colored_chars[i].last().unwrap()];
        for j in 0..colored_chars[i].len() {
            std::mem::swap(&mut s[colored_chars[i][j]], &mut written);
        }
    }

    println!("{}", s.iter().collect::<String>());
}
