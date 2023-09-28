// 5min -> 5min

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        an: [u64; n],
    }

    let mut ans = 0;
    for a in 0..n {
        for b in a + 1..n {
            let ab = (an[a] * an[b]) % p;
            for c in b + 1..n {
                let abc = (ab * an[c]) % p;
                for d in c + 1..n {
                    let abcd = (abc * an[d]) % p;
                    for e in d + 1..n {
                        if (abcd * an[e]) % p == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
