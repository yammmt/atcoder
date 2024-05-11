use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
    }

    // O(n^2) で通る
    let mut cn = vec![0; n];
    for _ in 0..t {
        input! {
            pn: [usize; n],
        }
        for i in 0..n {
            cn[i] = cn[i].max(pn[i]);
        }
        println!("{}", cn.iter().sum::<usize>());
    }
}
