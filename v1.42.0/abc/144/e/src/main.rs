// 12min

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut an: [i64; n],
        mut ffn: [i64; n],
    }
    an.sort();
    ffn.sort();
    an.reverse();

    // K が小さければ Priority Queue で貪欲でも？
    let mut pass = 0;
    for i in 0..n {
        pass = pass.max(an[i] * ffn[i]);
    }
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mut cur_k = 0;
        for i in 0..n {
            cur_k += ((an[i] * ffn[i] - mid + ffn[i] - 1) / ffn[i]).max(0) as usize;
        }

        if cur_k <= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
