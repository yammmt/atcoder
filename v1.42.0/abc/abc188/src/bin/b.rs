use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
        bn: [i64; n],
    }
    let mut ans = 0;
    for i in 0..n {
        ans += an[i] * bn[i];
    }
    println!(
        "{}",
        if ans == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
