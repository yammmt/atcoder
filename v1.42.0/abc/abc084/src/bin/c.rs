// 15min

use proconio::input;

fn main() {
    input! {
        n: usize,
        csfn: [(u64, u64, u64); n - 1],
    }

    for i in 0..n {
        let mut ans = 0;
        for csf in &csfn[i..] {
            ans = ans.max(csf.1);
            if ans % csf.2 != 0 {
                ans += csf.2 - (ans % csf.2);
            }
            ans += csf.0;
            // println!("{} 着: {}", j + 1, ans);
        }
        println!("{}", ans);
    }
}
