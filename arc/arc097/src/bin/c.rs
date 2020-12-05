use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let mut vstr = vec![];
    let mut hs = HashSet::new();
    for c in 0..27 {
        for i in 0..s.len() {
            if s[i] as u8 - b'a' != c {
                continue;
            }

            if !hs.contains(&s[i].to_string()) {
                hs.insert(s[i].to_string());
                vstr.push(s[i].to_string());
            }

            let mut curc = vec![s[i]];
            for j in i + 1..(i + k).min(s.len()) {
                curc.push(s[j]);
                let curstr = curc.iter().collect::<String>();
                if !hs.contains(&curstr) {
                    hs.insert(curstr.clone());
                    vstr.push(curstr);
                }
            }
        }
        if vstr.len() >= k {
            vstr.sort();
            println!("{}", vstr[k - 1]);
            return;
        }
    }
    // println!("{:?}", vstr);
}
