use proconio::input;
use std::cmp::Ordering;

fn divisor_num(n: u64) -> u64 {
    let mut ret = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            ret += 1;
            if n / i != i {
                ret += 1;
            }
        }
        i += 1;
    }

    ret
}

fn main() {
    input! {
        x: u64,
        y: u64,
    }

    let x_pts = divisor_num(x);
    let y_pts = divisor_num(y);

    println!(
        "{}",
        match x_pts.cmp(&y_pts) {
            Ordering::Greater => "X",
            Ordering::Less => "Y",
            Ordering::Equal => "Z",
        }
    );
}
