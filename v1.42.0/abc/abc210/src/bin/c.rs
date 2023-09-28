use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        cn: [usize; n],
    }

    let mut hm = HashMap::new();
    for c in cn.iter().take(k) {
        let cnt = hm.entry(*c).or_insert(0);
        *cnt += 1;
    }

    let mut ans = 0;
    for i in k..n + 1 {
        // println!("{:?}", hm);
        ans = ans.max(hm.len());
        let cnt = hm.get_mut(&cn[i - k]).unwrap();
        if *cnt > 1 {
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
