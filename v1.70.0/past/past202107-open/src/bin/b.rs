use proconio::input;

fn main() {
    input! {
        mut a: u64,
        b: u64,
        c: u64,
    }

    while a > b * c {
        a -= 1;
    }

    println!("{}", a as f64 / b as f64);
}
