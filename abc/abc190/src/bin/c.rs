// WA: 変数名変更し忘れ (+8min) :hankey:

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
        k: usize,
        cdk: [(usize, usize); k],
    }

    let mut ans = 0;
    for bit_row in 0..2u32.pow(k as u32) {
        let mut use_c = vec![];
        for i in 0..k {
            if bit_row & (1 << i) > 0 {
                use_c.push(i);
            }
        }
        // println!("{:?}", use_c);
        let mut cur = vec![0; n];
        for i in 0..k {
            if use_c.contains(&i) {
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
