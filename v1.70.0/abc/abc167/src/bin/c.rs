use proconio::fastout;
use proconio::input;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
    }
    let mut c_am = vec![];
    for _ in 0..n {
        input! {
            c: usize,
            am: [usize; m],
        }
        c_am.push((c, am));
    }

    let mut ans = DUMMY;
    for bit in 0..2u32.pow(n as u32) {
        let mut b = bit;
        let mut cost = 0;
        let mut understanding = vec![0; m];
        for c_a in &c_am {
            if b & 1 == 1 {
                cost += c_a.0;
                for (j, u) in understanding.iter_mut().enumerate() {
                    *u += c_a.1[j];
                }
            }
            b /= 2;
        }

        if understanding.iter().all(|&u| u >= x) {
            ans = ans.min(cost);
        }
    }

    println!("{}", if ans == DUMMY { -1 } else { ans as isize });
}
