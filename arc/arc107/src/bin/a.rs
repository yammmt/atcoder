use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    let d = 998_244_353;

    let aa = ((1 + a) * a / 2) % d;
    let bb = ((1 + b) * b / 2) % d;
    let cc = ((1 + c) * c / 2) % d;

    println!(
        "{}",
        ((aa * bb) % d * cc) % d
    );
}
