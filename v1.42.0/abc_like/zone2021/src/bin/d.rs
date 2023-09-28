// use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        s: Chars,
    }

    let mut vdq = VecDeque::new();
    let mut reversed = false;
    for c in &s {
        if *c == 'R' {
            reversed = !reversed;
            continue;
        }

        if reversed {
            if vdq.is_empty() {
                vdq.push_front(*c);
            } else {
                let prev = vdq.pop_front().unwrap();
                if *c != prev {
                    vdq.push_front(prev);
                    vdq.push_front(*c);
                }
            }
        } else if vdq.is_empty() {
            vdq.push_back(*c);
        } else {
            let prev = vdq.pop_back().unwrap();
            if  *c != prev {
                vdq.push_back(prev);
                vdq.push_back(*c);
            }
        }
    }

    let mut ans = vec![];
    if reversed {
        while let Some(cur) = vdq.pop_back() {
            ans.push(cur);
        }
    } else {
        while let Some(cur) = vdq.pop_front() {
            ans.push(cur);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
