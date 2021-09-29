use proconio::input;

fn to_decimal(mut a: i64, k: i64) -> i64 {
    let mut ret = 0;
    let mut cur = 1;
    while a > 0 {
        ret += cur * (a % 10);
        cur *= k;
        a /= 10;
    }
    ret
}

fn main() {
    input! {
        k: i64,
        a: i64,
        b: i64,
    }
    // println!("{}", to_decimal(a, k));
    println!("{}", to_decimal(a, k) * to_decimal(b, k));
}
