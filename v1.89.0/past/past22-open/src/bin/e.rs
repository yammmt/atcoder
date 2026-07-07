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

    let upper_bound = |b| {
        let mut pass = 0;
        let mut fail = n;
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            if an[mid] <= b {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass as isize
    };

    let mut ans = 0;
    let mut pos_last = upper_bound(bm[0]);
    for &b in bm.iter().skip(1) {
        let pos_cur = upper_bound(b);
        ans += (pos_cur - pos_last).unsigned_abs();
        pos_last = pos_cur;
    }

    println!("{ans}");
}
