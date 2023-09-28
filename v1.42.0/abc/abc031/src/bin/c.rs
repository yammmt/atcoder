// 制約が緩い

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut ans = std::i64::MIN;
    for tkhs in 0..n {
        let mut aoki_max = std::i64::MIN;
        let mut aoki_i = 0;
        for aoki in 0..n {
            if aoki == tkhs {
                continue;
            }

            // println!("t: {}, a: {}", tkhs, aoki);
            let b = tkhs.min(aoki);
            let e = tkhs.max(aoki);
            let mut cur = 0;
            for i in b..e + 1 {
                if (i - b) % 2 == 1 {
                    // println!("  += {}", an[i]);
                    cur += an[i];
                }
            }
            // println!("  ap: {}", cur);
            if cur > aoki_max {
                aoki_max = cur;
                aoki_i = aoki;
            }
        }
        // println!("    a_i: {}", aoki_i);

        let b = tkhs.min(aoki_i);
        let e = tkhs.max(aoki_i);
        let mut cur = 0;
        for i in b..e + 1 {
            if (i - b) % 2 == 0 {
                cur += an[i];
            }
        }
        // println!("    t_p: {}", cur);
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
