// WA: ans オーバーフロー

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
        cn: [usize; n],
    }

    let mut a_cnt = vec![0; n + 1];
    for a in &an {
        a_cnt[*a] += 1;
    }

    let mut ans = 0i64;
    for c in &cn {
        ans += a_cnt[bn[*c - 1]];
    }

    println!("{}", ans);
}
