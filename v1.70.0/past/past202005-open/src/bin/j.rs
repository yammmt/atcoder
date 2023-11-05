use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        am: [usize; m],
    }

    // 美味しさが降順に並ぶので二分探索が使える
    let mut an_max = vec![0; n];
    for a in &am {
        let mut pass = n as isize;
        let mut fail = -1;
        while pass - fail > 1 {
            let mid = ((pass + fail) / 2) as usize;
            if an_max[mid] < *a {
                pass = mid as isize;
            } else {
                fail = mid as isize;
            }
        }
        if pass == n as isize {
            println!("-1");
        } else {
            println!("{}", pass + 1);
            an_max[pass as usize] = *a;
        }
    }
}
