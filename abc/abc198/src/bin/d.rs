// :fu: :fu: :fu: 21-04

// use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
use permutohedron::heap_recursive;

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
    }

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

    let mut char_idx = vec![999; 26];
    let mut j = 0;
    for (i, ap) in appeared.iter().enumerate() {
        if !ap {
            continue;
        }

        char_idx[i] = j;
        j += 1;
    }
    // println!("cidx: {:?}", char_idx);
    let mut char_idx_idx = vec![999; 26];
    let mut j = 0;
    for i in 0..char_idx.len() {
        if char_idx[i] == 999 {
            continue;
        }

        char_idx_idx[j] = i;
        j += 1;
    }
    // println!("{:?}", char_idx_idx);
    // return;

    let mut searched = HashSet::new();
    let mut data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    heap_recursive(&mut data, |p| {
        let mut orders = vec![];
        orders.push(p.to_vec());

        for oo in &orders {
            // println!("{:?}", oo);
            let mut cur_map = vec![999; 26];
            for i in 0..char_kinds {
                // println!("  i: {}", i);
                cur_map[char_idx_idx[i]] = oo[i];
            }

            if !searched.contains(&cur_map) {
                // println!("{:?}", cur_map);
                searched.insert(cur_map.clone());
                // w: 先頭 0 は弾く
                if cur_map[(s1[0] as u8 - b'a') as usize] == 0
                    || cur_map[(s2[0] as u8 - b'a') as usize] == 0
                    || cur_map[(s3[0] as u8 - b'a') as usize] == 0
                {
                    let mut new_s1 = 0i64;
                    let mut new_s2 = 0i64;
                    let mut new_s3 = 0i64;
                    for s in &s1 {
                        new_s1 *= 10;
                        new_s1 += cur_map[(*s as u8 - b'a') as usize];
                    }
                    for s in &s2 {
                        new_s2 *= 10;
                        new_s2 += cur_map[(*s as u8 - b'a') as usize];
                    }
                    for s in &s3 {
                        new_s3 *= 10;
                        new_s3 += cur_map[(*s as u8 - b'a') as usize];
                    }
                    if new_s1 + new_s2 == new_s3 {
                        println!("{}", new_s1);
                        println!("{}", new_s2);
                        println!("{}", new_s3);
                        return;
                    }
                }
            }
        }
    });
    // println!("{:?}", orders);
    // return;


    // for oo in &orders {
    //     // println!("{:?}", oo);
    //     let mut cur_map = vec![999; 26];
    //     for i in 0..char_kinds {
    //         // println!("  i: {}", i);
    //         cur_map[char_idx_idx[i]] = oo[i];
    //     }

    //     if searched.contains(&cur_map) {
    //         continue;
    //     }
    //     // println!("{:?}", cur_map);

    //     searched.insert(cur_map.clone());
    //     // w: 先頭 0 は弾く
    //     if cur_map[(s1[0] as u8 - b'a') as usize] == 0
    //         || cur_map[(s2[0] as u8 - b'a') as usize] == 0
    //         || cur_map[(s3[0] as u8 - b'a') as usize] == 0
    //     {
    //         continue;
    //     }

    //     let mut new_s1 = 0i64;
    //     let mut new_s2 = 0i64;
    //     let mut new_s3 = 0i64;
    //     for s in &s1 {
    //         new_s1 *= 10;
    //         new_s1 += cur_map[(*s as u8 - b'a') as usize];
    //     }
    //     for s in &s2 {
    //         new_s2 *= 10;
    //         new_s2 += cur_map[(*s as u8 - b'a') as usize];
    //     }
    //     for s in &s3 {
    //         new_s3 *= 10;
    //         new_s3 += cur_map[(*s as u8 - b'a') as usize];
    //     }
    //     if new_s1 + new_s2 == new_s3 {
    //         println!("{}", new_s1);
    //         println!("{}", new_s2);
    //         println!("{}", new_s3);
    //         return;
    //     }
    //     // println!("{}", new_s1);
    //     // println!("{}", new_s2);
    //     // println!("{}", new_s3);
    //     // return;
    // }

    println!("UNSOLVABLE");
}
