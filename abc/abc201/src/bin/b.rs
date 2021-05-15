use proconio::input;

fn main() {
    input! {
        n: usize,
        mut stn: [(String, i64); n],
    }
    stn.sort_by(|a, b| {
        (a.1).cmp(&b.1)
    });
    stn.reverse();
    println!("{}", stn[1].0);
}
