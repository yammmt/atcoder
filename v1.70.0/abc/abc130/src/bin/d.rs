use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    let mut cur = 0;
    let mut r = 0;
    let mut ans = 0;
    for l in 0..n {
        while r < n && cur < k {
            cur += an[r];
            r += 1;
        }
        if cur < k {
            break;
        }

        // 終点の位置が r 以上すべて
        ans += n - r + 1;
        cur -= an[l];
    }

    println!("{ans}");
}
