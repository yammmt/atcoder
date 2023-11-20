// 頭ぐしゃぐしゃ系の中でも最悪

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut an: [usize; n],
        mut bm: [usize; m],
    }
    an.sort_unstable();
    bm.sort_unstable();

    let mut pass = (*an.last().unwrap()).max(*bm.last().unwrap()) + 1;
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mid = 100;
        println!("mid: {mid}");
        // TODO: mid より小さな値の個数を返しているわけで逆では
        // mid で売るのは an[i] が mid 以下となる場合のみ
        // lower_bound で要素数を数える
        let mut pass_s = n as isize;
        let mut fail_s = -1;
        while pass_s - fail_s > 1 {
            let mid_s = (pass_s + fail_s) / 2;
            if an[mid_s as usize] > mid {
                pass_s = mid_s;
            } else {
                fail_s = mid_s;
            }
        }
        let sellers = pass_s;

        // mid で買うのは bm[i] が mid 以下となる？
        let mut pass_b = m as isize;
        let mut fail_b = -1;
        while pass_b - fail_b > 1 {
            let mid_b = (pass_b + fail_b) / 2;
            if bm[mid_b as usize] >= mid {
                pass_b = mid_b;
            } else {
                fail_b = mid_b;
            }
        }
        // mid より小さい人たちは買わないので
        let buyers = m as isize - pass_b;

        if sellers >= buyers {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{pass}");
}
