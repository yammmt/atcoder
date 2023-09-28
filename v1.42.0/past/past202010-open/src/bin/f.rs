use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        sn: [String; n],
    }

    let mut hm = HashMap::new();
    for s in &sn {
        let cnt = hm.entry(s).or_insert(0);
        *cnt += 1;
    }

    let mut ans = vec![];
    for (k, v) in &hm {
        ans.push((v, k));
    }
    ans.sort_unstable();
    ans.reverse();
    // println!("{:?}", ans);

    if (k != 1 && ans[k - 2].0 == ans[k - 1].0) || (k != ans.len() && ans[k - 1].0 == ans[k].0) {
        println!("AMBIGUOUS");
    } else {
        println!("{}", ans[k - 1].1);
    }
}
