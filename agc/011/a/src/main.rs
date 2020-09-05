// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: u64,
        k: u64,
        mut t: [u64; n],
    }

    t.sort();
    let mut ans = 0;
    let mut humans = 0;
    let mut t_limit = t[0] + k;
    for i in 0..n {
        // 定員 or 待てないので出発
        if humans == c || t[i] > t_limit {
            ans += 1;
            humans = 0;
        }

        humans += 1;
        // 一人目: 出発時刻は限界まで待つ
        if humans == 1 {
            t_limit = t[i] + k;
        }

        // 最後は端数となり得る
        if i == n - 1 && humans > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
