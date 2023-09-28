use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        sn: [String; n],
        m: usize,
        tm: [String; m],
    }

    let mut scores = HashMap::new();
    for s in &sn {
        let cnt = scores.entry(s).or_insert(0isize);
        *cnt += 1;
    }
    for t in &tm {
        let cnt = scores.entry(t).or_insert(0);
        *cnt -= 1;
    }

    let mut ans = 0;
    for v in scores.values() {
        ans = ans.max(*v);
    }
    println!("{}", ans);
}
