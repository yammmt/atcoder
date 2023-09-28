use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        _c: i64,
        k: i64,
    }

    if a + b >= k {
        println!("{}", a.min(k));
    } else {
        println!("{}", a - (k - a - b));
    }
}
