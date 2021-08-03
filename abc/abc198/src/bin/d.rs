// :fu: :fu: :fu: 21-04 実装が重い
// Rust の permutation/clojure の仕様で詰んだしオーバーフロー見落として泥沼

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        mut s3: [Chars; 3],
    }
    s3.iter_mut().for_each(|s| s.reverse());

    let mut abc = vec![];
    for s in &s3 {
        for c in s {
            abc.push(*c);
        }
    }
    abc.sort_unstable();
    abc.dedup();
    if abc.len() > 10 || s3[0].len().max(s3[1].len()) > s3[2].len() {
        println!("UNSOLVABLE");
        return;
    }

    for perm in (0..10).permutations(abc.len()) {
        let mut hm = HashMap::new();
        for i in 0..abc.len() {
            hm.insert(abc[i], perm[i]);
        }

        // 先頭 0 は拒否
        if *hm.get(&s3[0][s3[0].len() - 1]).unwrap() == 0
            || *hm.get(&s3[1][s3[1].len() - 1]).unwrap() == 0
            || *hm.get(&s3[2][s3[2].len() - 1]).unwrap() == 0
        {
            continue;
        }

        let mut pass = true;
        let mut add_one = 0;
        for i in 0..s3[2].len() {
            let n0 = if i < s3[0].len() {
                *hm.get(&s3[0][i]).unwrap()
            } else {
                0
            };
            let n1 = if i < s3[1].len() {
                *hm.get(&s3[1][i]).unwrap()
            } else {
                0
            };
            let n2 = *hm.get(&s3[2][i]).unwrap();
            let added = n0 + n1 + add_one;
            if added % 10 != n2 {
                pass = false;
                break;
            }
            add_one = added / 10;
        }

        if pass && add_one == 0 {
            s3.iter_mut().for_each(|s| s.reverse());
            for s in &s3 {
                for c in s {
                    print!("{}", hm.get(c).unwrap());
                }
                println!();
            }
            return;
        }
    }

    println!("UNSOLVABLE");
}
