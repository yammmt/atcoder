// :fu: 22-03 やる気がない

use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    let m = 998_244_353;

    let mut digits = 0;
    let mut nn = n;
    while nn > 0 {
        digits += 1;
        nn /= 10;
    }

    let mut ans = 0u128;

    let mut ten = 10;
    for _ in 0..digits - 1 {
        let a_first = 1;
        let a_last = ten - ten / 10;
        let a_len = a_last - a_first + 1;
        // println!("{} {} {}", a_first, a_last, a_len);

        ans = (ans + (a_first + a_last) * a_len / 2) % m;

        ten *= 10;
    }
    // println!("{}", ans);

    let a_first = 1;
    let a_last = n - ten / 10 + 1;
    let a_len = a_last - a_first + 1;
    // println!("{} {} {}", a_first, a_last, a_len);
    ans = (ans + (a_first + a_last) * a_len / 2) % m;

    println!("{}", ans);
}
