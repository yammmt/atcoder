// 14.5min

use proconio::input;

fn main() {
    input! {
        q: usize,
        lrq: [(usize, usize); q],
    }

    let mut is_prime = vec![true; 100000 + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 2;
    while p * p <= 100000 {
        // println!("p: {}", p);
        if !is_prime[p] {
            p += 1;
            continue;
        }

        let mut x = 2;
        while x * p <= 100000 {
            // println!("x: {}", x);
            is_prime[x * p] = false;
            x += 1;
        }
        p += 1;
    }

    let mut rsum = vec![0; 100000 + 1];
    let mut cursum = 0;
    for i in 1..=100000 {
        if i % 2 == 1 && is_prime[i] && is_prime[(i + 1) / 2] {
            cursum += 1;
        }
        rsum[i] = cursum;
    }

    for lr in &lrq {
        println!("{}", rsum[lr.1] - rsum[lr.0 - 1]);
    }
}
