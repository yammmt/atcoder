// (Editional) 掛け算の最大回数が大したことないことに気付くともっと簡単

use proconio::input;

fn repeat_square(n: u64, p: u64) -> u64 {
    if p == 0 {
        1
    } else if p == 1 {
        n
    } else if p % 2 == 0 {
        let rp = repeat_square(n, p / 2);
        let limit = 10u64.pow(9);
        if rp > limit || rp * rp > limit {
            limit
        } else {
            rp * rp
        }
    } else {
        let rp = repeat_square(n, p - 1);
        if rp > 10u64.pow(9) {
            10u64.pow(9)
        } else {
            n * rp
        }
    }
}

fn main() {
    input! {
        a: u64,
        r: u64,
        n: u64,
    }

    let rn = repeat_square(r, n - 1);
    if a * rn > 10u64.pow(9) {
        println!("large");
    } else {
        println!("{}", a * rn);
    }
}
