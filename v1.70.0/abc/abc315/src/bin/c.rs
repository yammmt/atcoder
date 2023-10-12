use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut fsn: [(usize, usize); n],
    }

    let mut ans = 0;

    // 同じ味を二つ
    let mut hm = HashMap::new();
    for fs in &fsn {
        let counter = hm.entry(fs.0).or_insert(vec![]);
        counter.push(fs.1);
    }
    for v in &hm {
        if v.1.len() >= 2 {
            let mut vv = v.1.clone();
            vv.sort_unstable();
            vv.reverse();
            ans = ans.max(vv[0] + vv[1] / 2);
        }
    }

    // 違う味を二つ
    fsn.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    fsn.reverse();
    for fs in fsn.iter().skip(1) {
        if fs.0 != fsn[0].0 {
            ans = ans.max(fsn[0].1 + fs.1);
            break;
        }
    }

    println!("{ans}");
}
