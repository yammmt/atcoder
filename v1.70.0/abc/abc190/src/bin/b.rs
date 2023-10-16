use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        xyn: [(usize, usize); n],
    }

    for xy in &xyn {
        if xy.0 < s && xy.1 > d {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
