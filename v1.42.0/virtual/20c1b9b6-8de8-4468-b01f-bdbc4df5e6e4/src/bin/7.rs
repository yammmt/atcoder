use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        _c: isize,
        k: isize,
    }

    println!(
        "{}",
        if k <= a {
            k
        } else if k <= a + b {
            a
        } else {
            a - (k - a - b)
        }
    );
}
