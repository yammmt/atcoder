// しゃくとり法

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    let mut ans = 0;
    let mut hm = HashMap::new();
    let mut l_i = 0;
    for (i, a) in an.iter().enumerate() {
        let cnt = hm.entry(a).or_insert(0);
        *cnt += 1;

        while hm.len() > k {
            // 汚いが
            let cnt = hm.entry(&an[l_i]).or_insert(1);
            *cnt -= 1;
            if *cnt == 0 {
                hm.remove(&an[l_i]);
            }
            l_i += 1;
        }
        // println!("[{}, {}]: {:?}", l_i, i, hm);
        ans = ans.max(i - l_i + 1);
    }

    println!("{}", ans);
}
