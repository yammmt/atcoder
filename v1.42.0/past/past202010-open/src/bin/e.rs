use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let s_s = s.iter().collect::<String>();

    for perm in (0..n).permutations(n) {
        let mut cur = vec![];
        for p in &perm {
            cur.push(s[*p]);
        }
        let t_s = cur.iter().collect::<String>();
        cur.reverse();
        let t_s_r = cur.iter().collect::<String>();
        if t_s != s_s && t_s_r != s_s {
            println!("{}", t_s);
            return;
        }
    }

    println!("None");
}
