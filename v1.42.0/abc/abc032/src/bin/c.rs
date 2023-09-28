use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        sn: [i64; n],
    }

    if sn.iter().any(|a| *a == 0) {
        println!("{}", n);
        return;
    }

    let mut ans = 0;
    let mut cur = 1;
    let mut right = 0;
    for left in 0..n {
        while right < n && cur * sn[right] <= k {
            cur *= sn[right];
            right += 1;
        }
        ans = ans.max(right - left);
        if left == right {
            right += 1;
        } else {
            cur /= sn[left];
        }
    }

    println!("{}", ans);
}
