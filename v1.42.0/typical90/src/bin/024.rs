use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i64; n],
        bn: [i64; n],
    }
    let mut lowk = 0;
    for i in 0..n {
        lowk += (an[i] - bn[i]).abs() as usize;
    }
    println!(
        "{}",
        if lowk <= k && (k - lowk) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
