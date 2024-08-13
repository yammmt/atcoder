use proconio::fastout;
use proconio::input;

const DUMMY: usize = usize::MAX / 3;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
    }

    let mut pass = 0;
    let mut fail = DUMMY;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        let mut cost = 0;
        for &a in &an {
            cost += a.min(mid);
        }
        if cost <= m {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    if fail == DUMMY {
        println!("infinite");
        return;
    }

    println!("{pass}");
}
