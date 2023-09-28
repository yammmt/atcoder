// -*- coding:utf-8-unix -*-

// :fu: (二次元累積和探す)
// 気付けば数問 数学でなく算数寄り

// > 70min, 算数に > 50 min

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        lrm: [(usize, usize); m],
        pqq: [(usize, usize); q],
    }

    let mut imosbase = vec![vec![0; n + 1]; n + 1];
    for lr in &lrm {
        imosbase[lr.0][lr.1] += 1;
    }

    let mut imosx = vec![vec![0; n + 1]; n + 1];
    for i in 0..n + 1 {
        let mut cur = 0;
        for j in 0..n + 1 {
            cur += imosbase[i][j];
            imosx[i][j] = cur;
        }
    }
    let mut imossum = vec![vec![0; n + 1]; n + 1];
    for j in 0..n + 1{
        let mut cur = 0;
        for i in 0..n + 1 {
            cur += imosx[i][j];
            imossum[i][j] = cur;
        }
    }
    // println!("{:?}", imossum);

    for pq in &pqq {
        // println!("[{:?}]", pq);
        println!(
            "{}",
            imossum[pq.1][pq.1]
                - imossum[pq.0 - 1][pq.1] // 始点の手前まで
                - imossum[pq.1][pq.0 - 1] //対応する言語が思い浮かばない
                + imossum[pq.0 - 1][pq.0 - 1]
        );
    }
}
