// WA: 変数名変更し忘れ (+8min) :hankey:

use proconio::input;

fn bit_rows(n: u32) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    for b in 0..2u64.pow(n) {
        let mut cur = vec![];
        for i in 0..n {
            if b & (1 << i) > 0 {
                cur.push(i as usize);
            }
        }
        ret.push(cur);
    }
    ret
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
        k: usize,
        cdk: [(usize, usize); k],
    }

    let mut ans = 0;
    let ptrns = bit_rows(k as u32);
    for p in &ptrns {
        let mut cur = vec![0; n];
        for i in 0..k {
            if p.contains(&i) {
                cur[cdk[i].0 - 1] += 1;
            } else {
                cur[cdk[i].1 - 1] += 1;
            }
        }
        let mut cur_ans = 0;
        for ab in &abm {
            if cur[ab.0 - 1] > 0 && cur[ab.1 - 1] > 0 {
                cur_ans += 1;
            }
        }
        // println!("{}", cur_ans);
        ans = ans.max(cur_ans);
    }

    println!("{}", ans);
}
