// :fu: :fu: :fu: 21-04 実装が重い
// Rust の permutation/clojure の仕様で詰んだしオーバーフロー見落として泥沼

use proconio::input;
use permutohedron::heap_recursive;

fn solve(s1: Vec<char>, s2: Vec<char>, s3: Vec<char>) {
    let mut appeared = vec![false; 26];
    for s in &s1 {
        appeared[(*s as u8 - b'a') as usize] = true;
    }
    for s in &s2 {
        appeared[(*s as u8 - b'a') as usize] = true;
    }
    for s in &s3 {
        appeared[(*s as u8 - b'a') as usize] = true;
    }
    let char_kinds = appeared.iter().filter(|&t| *t).count();
    if char_kinds > 10 {
        println!("UNSOLVABLE");
        return;
    }

    let mut cleared = false;
    let mut data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    heap_recursive(&mut data, |p| {
        if !cleared {
            let mut char_to_num = vec![999i64; 26];
            let mut ci = 0;
            for nn in p {
                while ci < 26 && !appeared[ci] {
                    ci += 1;
                }
                if ci == 26 {
                    break;
                }

                char_to_num[ci] = *nn;
                ci += 1;
            }
            // println!("{:?}", char_to_num);

            // 先頭 0 は省く
            if  char_to_num[(s1[0] as u8 - b'a') as usize] != 0
                && char_to_num[(s2[0] as u8 - b'a') as usize] != 0
                && char_to_num[(s3[0] as u8 - b'a') as usize] != 0
            {
                let mut s1_new = 0;
                let mut s2_new = 0;
                let mut s3_new = 0;
                for s in &s1 {
                    s1_new *= 10;
                    s1_new += char_to_num[(*s as u8 - b'a') as usize];
                }
                for s in &s2 {
                    s2_new *= 10;
                    s2_new += char_to_num[(*s as u8 - b'a') as usize];
                }
                for s in &s3 {
                    s3_new *= 10;
                    s3_new += char_to_num[(*s as u8 - b'a') as usize];
                }

                if s1_new + s2_new == s3_new {
                    println!("{}", s1_new);
                    println!("{}", s2_new);
                    println!("{}", s3_new);
                    cleared = true;
                    // return; // できない
                }
            }
        }
    });

    if !cleared {
        println!("UNSOLVABLE");
    }
}

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
    }
    let vs1 = s1.chars().collect::<Vec<char>>();
    let vs2 = s2.chars().collect::<Vec<char>>();
    let vs3 = s3.chars().collect::<Vec<char>>();
    solve(vs1, vs2, vs3);
}
