use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
        bn: [i64; n],
    }
    let mut amax = 0;
    let mut bmin = bn[0];
    for i in 0..n {
        amax = amax.max(an[i]);
        bmin = bmin.min(bn[i]);
    }
    println!("{}", (bmin - amax + 1).max(0));
}
