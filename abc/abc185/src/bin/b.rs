use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: i64,
        m: usize,
        t: i64,
        abm: [(i64, i64); m],
    }

    let mut nn = n;
    let mut prev = 0;
    for i in 0..m {
        nn -= abm[i].0 - prev;
        // println!("n1: {}", nn);
        if nn <= 0 {
            println!("No");
            return;
        }
        // println!("{}", nn + abm[i].1 - abm[i].0);
        // println!("{}", nn);
        nn = (nn + abm[i].1 - abm[i].0).min(n);
        prev = abm[i].1;
        // println!("n2: {}", nn);
    }

    if nn > t - prev {
        println!("Yes");
    } else {
        println!("No");
    }
}
