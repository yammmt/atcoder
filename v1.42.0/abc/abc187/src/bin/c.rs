// 正直問題文よくわからん "!!" 付きは考慮するの？

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

    for s in &sn {
        // if *hm.get(&s).unwrap() == 1 {
        //     continue;
        // }

        let mut vs = s.chars().collect::<Vec<char>>();
        vs.insert(0, '!');
        let curs = vs.iter().collect::<String>();
        if hm.contains_key(&curs) {
            println!("{}", s);
            return;
        }
    }

    println!("satisfiable");
}
