use proconio::input;

fn main() {
    input! {
        s: i64,
        t: i64,
    }

    let mut ans = 0;
    // for aa in 0..s + 1 {
    //     let a = aa as i64;
    //     for bb in 0..s + 1 {
    //         let b = bb as i64;
    //         println!("{} {}", a, b);
    //         let c_from_s = s - a - b + 1;
    //         let c_from_t = if a == 0 || b == 0 {
    //             s
    //         } else {
    //             t / a / b
    //         };
    //         println!("  {} {}", c_from_s, c_from_t);
    //         ans += (c_from_s.min(c_from_t)).max(0);
    //     }
    // }
    for a in 0..s + 1 {
        for b in 0..s + 1 {
            for c in 0..s + 1 {
                if a + b + c <= s && a * b * c <= t {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
