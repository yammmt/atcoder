use proconio::input;
use proconio::marker::{Bytes, Chars};

fn main() {
    input! {
        x: Bytes,
        n: usize,
        mut sn: [Chars; n],
    }

    let mut priority = vec![0; 26];
    for (i, xx) in x.iter().enumerate() {
        priority[(*xx - b'a') as usize] = i;
    }

    sn.sort_unstable_by(|va, vb| {
        let len_limit = va.len().min(vb.len());
        for i in 0..len_limit {
            let prio_a = priority[(va[i] as u8 - b'a') as usize];
            let prio_b = priority[(vb[i] as u8 - b'a') as usize];
            if prio_a != prio_b {
                return prio_a.cmp(&prio_b);
            }
        }
        va.len().cmp(&vb.len())
    });

    for s in &sn {
        println!("{}", s.iter().collect::<String>());
    }
}
