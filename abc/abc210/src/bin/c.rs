use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        cn: [usize; n],
    }

    let mut hm = HashMap::new();
    for i in 0..k {
        let cnt = hm.entry(cn[i]).or_insert(0);
        *cnt += 1;
    }

    let mut ans = 0;
    for i in k..n + 1 {
        // println!("{:?}", hm);
        ans = ans.max(hm.len());
        let cnt = hm.entry(cn[i - k]).or_insert(0);
        if cnt > &mut 1 {
            *cnt -= 1;
        } else {
            hm.remove(&cn[i - k]);
        }

        // println!("  {}", i);
        if i < n {
            let cnt = hm.entry(cn[i]).or_insert(0);
            *cnt += 1;
        }
    }

    println!("{}", ans);
}
