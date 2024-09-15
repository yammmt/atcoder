use proconio::fastout;
use proconio::input;

const MOD: usize = 998_244_353;

#[fastout]
fn main() {
    // 2^60 = 1152921504606846976
    // 2^60-1 = 1152921504606846975
    input! {
        mut n: usize,
        mut m: usize,
    }

    // m の立っているそれぞれの bit について [0, n] の範囲で何回立つかを求める
    // オーバーフロー何も考えずで通ったが, それでよいのか

    let mut ans = 0usize;
    let mut bit_cur = 1;
    while m > 0 {
        if m % 2 == 0 {
            m /= 2;
            n -= bit_cur.min(n);
            bit_cur *= 2;
            continue;
        }

        // println!("n: {n}, m: {m}, bit_cur: {bit_cur}");
        ans = (ans + bit_cur * (n / (bit_cur * 2))) % MOD;
        // println!("  {ans}");
        let fraction = (n % (bit_cur * 2)).min(bit_cur);
        ans = (ans + fraction) % MOD;
        // println!("  {ans}");

        m /= 2;
        n -= bit_cur.min(n);
        bit_cur *= 2;
    }

    println!("{ans}");
}
