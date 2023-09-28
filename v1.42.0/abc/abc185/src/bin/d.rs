use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut am: [usize; m],
    }
    if m == 0 {
        println!("1");
        return;
    }
    am.sort();

    let mut ranges = vec![];
    let mut blue_begin = 0;
    for i in 0..m {
        if am[i] - blue_begin - 1 > 0 {
            ranges.push(am[i] - blue_begin - 1);
        }
        blue_begin = am[i];
    }
    if blue_begin != n {
        ranges.push(n - blue_begin);
    }
    // println!("{:?}", ranges);

    if ranges.len() == 0 {
        println!("0");
        return;
    }

    let width = ranges.iter().min().unwrap();
    let mut ans = 0u64;
    for r in &ranges {
        ans += (*r / width) as u64;
        if *r % width != 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
