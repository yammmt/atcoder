use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: Usize1,
        sn: [String; n],
    }

    let mut cnt = HashMap::new();
    for s in &sn {
        let c = cnt.entry(s).or_insert(0);
        *c += 1;
    }

    let mut v = vec![];
    for (k, val) in cnt {
        v.push((Reverse(val), k));
    }
    v.sort_unstable();

    let mut can_ans = true;
    if k > 0 && v[k - 1].0 == v[k].0 {
        can_ans = false;
    }
    if k < v.len() - 1 && v[k + 1].0 == v[k].0 {
        can_ans = false;
    }

    if can_ans {
        println!("{}", v[k].1);
    } else {
        println!("AMBIGUOUS");
    }
}
