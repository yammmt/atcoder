use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        xn: [isize; n],
        pn: [usize; n],
        q: usize,
        lrq: [(isize, isize); q],
    }

    let mut pos_and_p = vec![(-1_000_000_001, 0)];
    for i in 0..n {
        pos_and_p.push((xn[i], pos_and_p.last().unwrap().1 + pn[i]));
    }
    pos_and_p.push((1_000_000_001, pos_and_p.last().unwrap().1));

    for (l, r) in lrq {
        // r 以下の全人数 - l 未満の全人数, を二分探索
        let mut pass = 0;
        let mut fail = pos_and_p.len();
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            if pos_and_p[mid].0 <= r {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let rp = pass;

        let mut pass = 0;
        let mut fail = pos_and_p.len();
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            if pos_and_p[mid].0 < l {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let lp = pass;

        println!("{}", pos_and_p[rp].1 - pos_and_p[lp].1);
    }
}
