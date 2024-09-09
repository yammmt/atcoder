use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        sn: [usize; n],
    }

    if sn.iter().any(|&s| s == 0) {
        println!("{n}");
        return;
    }

    let mut ans = 0;
    let mut cur = 1;
    let mut r = 0;
    for l in 0..n {
        if r < l {
            cur = 1;
            r = l;
        }

        // 条件を満たす限り掛けていく
        while r < n && cur * sn[r] <= k {
            cur *= sn[r];
            r += 1;
        }

        // r は条件を満たさない位置
        ans = ans.max(r - l);
        cur /= sn[l];
    }

    println!("{ans}");
}
