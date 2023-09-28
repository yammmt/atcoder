// NOT WORK

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        an: [u64; n],
        txyq: [(usize, usize, usize); q],
    }

    let mut ksum = vec![0; n + 1];
    for i in 0..n {
        ksum[i + 1] = an[i] ^ ksum[i];
    }

    let mut yi = vec![0u64; n + 1];
    for i in 0..q {
        if txyq[i].0 == 1 {
            yi[txyq[i].1] ^= txyq[i].2 as u64;
        } else {
            let mut ans = ksum[txyq[i].2] ^ ksum[txyq[i].1 - 1];
            // 高速に 区間内で置き換えられた数を全部 XOR 足す
            for i in txyq[i].1..txyq[i].2 + 1 {
                ans ^= yi[i];
            }
            println!("{}", ans);
        }
    }
}
