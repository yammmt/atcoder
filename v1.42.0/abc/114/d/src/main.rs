// 38min
// 数問 こわ

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut primes = vec![0u64; n + 1];
    for i in 2..n + 1 {
        let mut cur = i;
        let mut p = 2;
        while cur > 1 {
            if cur % p == 0 {
                primes[p] += 1;
                cur /= p;
            } else {
                p += 1;
            }
        }
    }
    // println!("{:?}", primes);

    let mut two = 0;
    let mut four = 0;
    let mut fourteen = 0;
    let mut twentyfour = 0;
    let mut seventyfour = 0;
    for p in &primes {
        if *p >= 74 {
            seventyfour += 1;
        } else if *p >= 24 {
            twentyfour += 1;
        } else if *p >= 14 {
            fourteen += 1;
        } else if *p >= 4 {
            four += 1;
        } else if *p >= 2 {
            two += 1;
        }
    }
    // println!("{}", two);
    // println!("{}", four);

    println!(
        "{}",
        // 3 * 5 * 5
        two * (four + fourteen + twentyfour + seventyfour) * (four + fourteen + twentyfour + seventyfour - 1) / 2
        + ((four + fourteen + twentyfour + seventyfour) * (four + fourteen + twentyfour + seventyfour - 1) * (four + fourteen + twentyfour + seventyfour - 2) / 2)
        // 5 * 15
        + (four * (fourteen + twentyfour + seventyfour))
        + ((fourteen + twentyfour + seventyfour) * (fourteen + twentyfour + seventyfour - 1))
        // 3 * 25
        + ((two + four + fourteen) * (twentyfour + seventyfour))
        + ((twentyfour + seventyfour) * (twentyfour + seventyfour - 1))
        // 1 * 75
        + seventyfour
    );
}
