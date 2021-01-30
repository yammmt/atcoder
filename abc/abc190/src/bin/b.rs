use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i64,
        d: i64,
        xyn: [(i64, i64); n],
    }

    for xy in &xyn {
        if xy.0 < s && xy.1 > d {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
