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
        a: u64,
        b: u64,
        c: u64,
    }

    let g = gcd(gcd(a, b), c);
    println!("{}", a / g - 1 + b / g - 1 + c / g - 1);
}
