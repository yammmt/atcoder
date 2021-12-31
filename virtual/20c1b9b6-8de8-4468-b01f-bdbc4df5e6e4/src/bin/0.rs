use proconio::input;

fn main() {
    input! {
        n: usize,
        ln: [i64; n],
    }
    let lmax = *ln.iter().max().unwrap();
    println!(
        "{}",
        if lmax < ln.iter().sum::<i64>() - lmax {
            "Yes"
        } else {
            "No"
        }
    );
}
