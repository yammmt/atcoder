// 行間は解説放送

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut divided_by_sq = (0..=n).collect::<Vec<usize>>();
    let mut i = 2;
    while i * i <= n {
        let x = i * i;
        let mut j = x;
        while j <= n {
            while divided_by_sq[j] % x == 0 {
                divided_by_sq[j] /= x;
            }
            j += x;
        }
        i += 1;
    }

    let mut cnt = vec![0; n + 1];
    for d in divided_by_sq {
        cnt[d] += 1;
    }

    let mut ans = 0;
    for c in cnt.iter().skip(1) {
        ans += c * c;
    }

    println!("{ans}");
}
