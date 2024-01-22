use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut an: [usize; n],
        xq: [usize; q],
    }
    an.sort_unstable();

    let mut cusum = vec![0; n + 1];
    for (i, a) in an.iter().enumerate() {
        cusum[i + 1] = cusum[i] + *a;
    }

    for x in xq {
        let mut i_greater = n as isize;
        let mut i_eq_or_lesser = -1;
        while (i_greater - i_eq_or_lesser) > 1 {
            let mid = (i_greater + i_eq_or_lesser) as usize / 2;
            if an[mid] > x {
                i_greater = mid as isize;
            } else {
                i_eq_or_lesser = mid as isize;
            }
        }

        let mut ans = 0;
        if i_eq_or_lesser >= 0 {
            let i = i_eq_or_lesser as usize;
            let cur = x * (i + 1) - cusum[i + 1];
            ans += cur;
        }
        if i_greater < n as isize {
            let i = i_greater as usize;
            let cur = (cusum[n] - cusum[i]) - x * (n - i);
            ans += cur;
        }
        println!("{ans}");
    }
}
