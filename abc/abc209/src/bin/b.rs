use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        an: [usize; n],
    }
    let asum = an.iter().sum::<usize>();


    println!(
        "{}",
        if asum - n / 2 <= x {
            "Yes"
        } else {
            "No"
        }
    );
}
