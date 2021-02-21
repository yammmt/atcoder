// :fu: 数問 21-02

use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut yakusu = vec![0; k + 1];
    for x in 1..k + 1 {
        // println!("x: {}", x);
        let mut i = 1;
        while i * i <= x {
            if x % i != 0 {
                i += 1;
                continue;
            }

            let b = x / i;
            let c = x / b;
            // println!("  {} {}", b, c);
            yakusu[x] += if b == c {
                1
            } else {
                2
            };

            i += 1;
        }
    }
    // println!("{:?}", yakusu);

    let mut ycusum = vec![0; k + 1];
    for i in 1..k + 1 {
        ycusum[i] = ycusum[i - 1] + yakusu[i];
    }
    // println!("{:?}", ycusum);

    let mut ans = 0;
    for a in 1..k + 1 {
        let bc = k / a;
        ans += ycusum[bc];
    }

    println!("{}", ans);
}
