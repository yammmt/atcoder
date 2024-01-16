use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        ln: [u64; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let l_ij = ln[i] + ln[j];
                let l_jk = ln[j] + ln[k];
                let l_ki = ln[k] + ln[i];
                if ln[i] != ln[j]
                    && ln[j] != ln[k]
                    && ln[k] != ln[i]
                    && l_ij > ln[k]
                    && l_jk > ln[i]
                    && l_ki > ln[j]
                {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}")
}
