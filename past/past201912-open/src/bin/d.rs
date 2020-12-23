use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: usize,
        an: [i32; n],
    }
    let mut cnt = vec![0; n + 1];
    for a in &an {
        cnt[*a as usize] += 1;
    }
    let mut ooi = 0;
    let mut sukunai = 0;
    for i in 1..n + 1 {
        if cnt[i] == 2 {
            ooi = i;
        } else if cnt[i] == 0 {
            sukunai = i;
        }
    }
    if ooi == 0 {
        println!("Correct");
    } else {
        println!("{} {}", ooi, sukunai);
    }
}
