use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        vpn: [(u64, u64); n],
    }

    let mut curx = 0;
    for i in 0..n {
        curx += vpn[i].0 * vpn[i].1;
        if curx > 100 * x {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
