use proconio::input;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * repeat_square(n, p - 1, m)) % m
    }
}

fn main() {
    input! {
        n: u64,
        k: u64,
    }
    let d = 1_000_000_007;

    if n == 1 {
        println!("{}", k);
        return;
    } else if n == 2 {
        println!("{}", (k * (k - 1)) % d);
        return;
    } else if n > 2 && k < 3 {
        println!("0");
        return;
    }

    let mut ans = (k * (k - 1)) % d;
    ans = (ans * repeat_square(k - 2, n - 2, d)) % d;

    println!("{}", ans);
}
