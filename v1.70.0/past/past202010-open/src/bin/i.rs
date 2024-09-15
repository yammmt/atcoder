use proconio::fastout;
use proconio::input;

const DUMMY: isize = isize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut an: [isize; n],
    }
    let a_sum = an.iter().sum::<isize>();
    for i in 0..n {
        an.push(an[i]);
    }

    // 尺取り
    let mut ans = DUMMY;
    let mut r = 0;
    let mut a_sum_cur = 0;
    for l in 0..n {
        // いる？
        if r < l {
            r = l + 1;
            a_sum_cur = an[l];
        }

        while r < l + n && a_sum_cur <= a_sum / 2 {
            a_sum_cur += an[r];
            let a_sum_left = a_sum - a_sum_cur;
            ans = ans.min((a_sum_left - a_sum_cur).abs());
            r += 1;
        }

        // println!("[{l}, {r})");
        a_sum_cur -= an[l];
    }

    println!("{ans}");
}
