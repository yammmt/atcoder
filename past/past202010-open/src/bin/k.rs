// 重実装

use proconio::input;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        k: usize,
    }
    let mut nk = vec![];
    let mut ank = vec![];
    for _ in 0..k {
        input! {
            n: usize,
        }
        nk.push(n);
        input! {
            an: [usize; n],
        }
        ank.push(an);
    }
    input! {
        q: usize,
        bq: [usize; q],
    }
    let d = 1_000_000_000;

    // a_b_j の値が高々 20 通りでしかなく、ループ内で全部走査しても間に合う
    // 定数倍が重めだからか時間制限が 5s

    // 数列単位で a_b_j 全数の出現回数を記憶しておく
    let mut include_a_num = vec![vec![0u64; 21]; k];
    for (i, an) in ank.iter().enumerate() {
        for a in an {
            include_a_num[i][*a] += 1;
        }
    }

    // 数列単位でその数列が出た場合に答えに加算される数を予め求めておく
    let mut ans_add = vec![vec![0u64; 21]; k];
    let mut self_add = vec![0; k];
    for (i, an) in ank.iter().enumerate() {
        let mut cur_cnt = vec![0; 21];
        for a in an {
            // 自身の数列を足す前に含まれている数に対して
            for j in a + 1..21 {
                ans_add[i][j] = (ans_add[i][j] + 1) % d;
            }
            // 自身に対して
            for cc in cur_cnt.iter().skip(*a + 1) {
                self_add[i] += *cc;
            }
            cur_cnt[*a] += 1;
        }
    }
    // println!("{:?}", ans_add);
    // println!("{:?}", self_add);
    // return;

    let mut ans = 0;
    let mut appears = vec![0; 21];
    // 実際に数え上げする
    for b in &bq {
        for (i, a) in ans_add[*b - 1].iter().enumerate() {
            ans = (ans + *a * appears[i]) % d;
        }
        ans = (ans + self_add[*b - 1]) % d;

        for i in 0..21 {
            appears[i] = (appears[i] + include_a_num[*b - 1][i]) % d;
        }
        // println!("  {}", ans);
    }

    println!("{}", ans);
}
