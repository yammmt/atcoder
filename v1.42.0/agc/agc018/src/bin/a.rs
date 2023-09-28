// 9min 1WA (等号判定)

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        k: u64,
        an: [u64; n],
    }

    if n == 1 {
        if an[0] == k {
            println!("POSSIBLE");
        } else {
            println!("IMPOSSIBLE");
        }
        return;
    }

    if *(an.iter().max().unwrap()) < k {
        println!("IMPOSSIBLE");
        return;
    }

    let mut allgcd = gcd(an[0], an[1]);
    if n > 2 {
        for a in &an[2..] {
            allgcd = gcd(allgcd, *a);
        }
    }
    if k % allgcd == 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
