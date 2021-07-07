// 苦手

use proconio::input;

fn main() {
    input! {
        mut p: usize,
    }

    let mut coins = vec![];
    let mut cur = 1;
    for i in 1..11 {
        cur *= i;
        coins.push(cur);
    }
    // println!("{:?}", coins);
    coins.reverse();

    let mut ans = 0;
    for c in &coins {
        // println!("{}", p);
        if p >= *c {
            let a = p / *c;
            ans += a;
            p -= a * *c;
        }
    }

    println!("{}", ans);
}
