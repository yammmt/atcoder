use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
        q: usize,
        qq: [(usize, usize, usize); q],
    }

    // (数, 位置) 昇順でソートすれば二分探索が使えるはず
    let mut num_and_pos = vec![];
    for (i, &a) in an.iter().enumerate() {
        num_and_pos.push((a, i + 1));
    }
    num_and_pos.sort_unstable();

    let lower_bound = |a| {
        let mut pass = -1;
        let mut fail = num_and_pos.len() as isize;
        while fail - pass > 1 {
            let mid = (pass + fail) as usize / 2;
            if num_and_pos[mid] < a {
                pass = mid as isize;
            } else {
                fail = mid as isize;
            }
        }
        pass
    };

    let upper_bound = |a| {
        let mut pass = -1;
        let mut fail = num_and_pos.len() as isize;
        while fail - pass > 1 {
            let mid = (pass + fail) as usize / 2;
            if num_and_pos[mid] <= a {
                pass = mid as isize;
            } else {
                fail = mid as isize;
            }
        }
        pass
    };

    for (l, r, x) in qq {
        let lb_l = lower_bound((x, l));
        let ub_r = upper_bound((x, r));
        println!("{}", ub_r - lb_l);
    }
}
