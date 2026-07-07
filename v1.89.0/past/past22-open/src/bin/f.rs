use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
        bm: [usize; m],
    }

    let mut i = 0;
    for b in bm {
        if an[i] == b {
            if i == n - 1 {
                println!("Yes");
                return;
            }

            i += 1;
        }
    }

    println!("No");
}
