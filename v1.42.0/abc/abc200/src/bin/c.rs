// 爆死した (WA 含め +30min ほど) が愚かな誤読由来であり復習するものでもない

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut ans = 0usize;
    let mut cnt = vec![0; 200];
    for a in &an {
        cnt[*a % 200] += 1;
    }

    for i in 0..200 {
        if cnt[i] > 1 {
            // nC2
            ans += (cnt[i] * (cnt[i] - 1)) / 2;
        }
    }

    println!("{}", ans);
}
