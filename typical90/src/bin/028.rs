// :fu: :fu: :fu: 21-04 座標で混乱して死ぬ とても嫌
// WA: 座標範囲

use proconio::input;

const SQ_SIZE: usize = 1001;

fn main() {
    input! {
        n: usize,
        lrn: [((usize, usize), (usize, usize)); n],
    }
    // println!("{:?}", lrn);

    let mut imos_w = vec![vec![0i64; SQ_SIZE]; SQ_SIZE];
    for lr in &lrn {
        for h in (lr.0).1..(lr.1).1 {
            imos_w[h][(lr.0).0] += 1;

            if (lr.1).0 < SQ_SIZE {
                imos_w[h][(lr.1).0] -= 1;
            }
        }
    }
    // println!("{:?}", imos_w);

    let mut ans = vec![0; n + 1];
    for i in 0..SQ_SIZE {
        let mut cur = 0;
        for j in 0..SQ_SIZE {
            cur += imos_w[i][j];
            ans[cur as usize] += 1;
        }
    }

    ans.iter().skip(1).for_each(|a| println!("{}", a));
}
