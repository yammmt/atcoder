use proconio::input;

const DUMMY: usize = usize::MAX / 4;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    // 最大長が pass 以下
    let mut pass = DUMMY;
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mut cur = 0;
        for a in &an {
            cur += ((*a + mid - 1) / mid) - 1;
        }
        if cur <= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{pass}");
}
