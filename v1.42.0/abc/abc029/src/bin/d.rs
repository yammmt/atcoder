// :fu: :fu: :fu: 21-05 ABC-C にありそうだが私上最悪に嫌な問題
// https://tsutaj.hatenablog.com/entry/2016/07/22/130031

use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut ans = 0;
    let mut div = 1;
    loop {
        // その桁の 1 は ptrn_len 長のうち div 個含まれる
        let mut cur = 0;
        let ptrn_len = div * 10;
        cur += (n / ptrn_len) * div;
        cur += (((n % ptrn_len) - div + 1).max(0)).min(div);

        // println!("  {}", cur);
        if cur == 0 {
            break;
        }

        ans += cur;
        div *= 10;
    }

    println!("{}", ans);
}
