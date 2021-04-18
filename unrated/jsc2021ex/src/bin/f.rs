use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }
    let mut hm = HashMap::new();
    for s in &sn {
        let cnt = hm.entry(s).or_insert(0);
        *cnt += 1;
    }
    let mut ansvec = vec![];
    for (k, v) in &hm {
        ansvec.push((v, k));
    }
    ansvec.sort();
    let ansnum = ansvec[ansvec.len() - 1].0;
    for a in &ansvec {
        if a.0 != ansnum {
            continue;
        }

        println!("{}", a.1);
    }
}
