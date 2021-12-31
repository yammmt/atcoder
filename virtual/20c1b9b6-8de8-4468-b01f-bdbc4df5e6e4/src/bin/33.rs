use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let mut ab = b;
    if b > 99 {
        ab += a * 1000;
    } else if b > 9 {
        ab += a * 100;
    } else {
        ab += a * 10;
    }

    let mut p = 1;
    while p * p <= ab {
        if p * p == ab {
            println!("Yes");
            return;
        }
        p += 1;
    }

    println!("No");
}
