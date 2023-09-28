// :fu: :fu: 21-07 方針は兎も角ひどくバグらせる

use proconio::input;

#[allow(clippy::needless_range_loop)]
fn count_no_more_than_p(dist: &Vec<Vec<isize>>, p: isize, x: isize) -> usize {
    let n = dist[0].len();
    let mut cur_dist = dist.clone();
    for i in 0..n {
        for j in 0..n {
            if cur_dist[i][j] == -1 {
                cur_dist[i][j] = x;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                cur_dist[i][j] = cur_dist[i][j].min(cur_dist[i][k] + cur_dist[k][j]);
            }
        }
    }

    let mut ret = 0;
    for i in 0..n {
        for j in i + 1..n {
            if cur_dist[i][j] <= p {
                ret += 1;
            }
        }
    }
    ret
}

fn main() {
    input! {
        n: usize,
        p: isize,
        k: usize,
        ann: [[isize; n]; n],
    }

    // 交通費 x を小さくすると街の移動距離も下がる
    // x を巨大数 (p + 1 で十分) としても条件を満たすならば Infinity になる
    // Infinity でなければ二分探索を頑張って upper_bound - lower_bound + 1 が答え

    if count_no_more_than_p(&ann, p, p + 1) == k {
        println!("Infinity");
        return;
    }

    let mut pass = p + 1;
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if count_no_more_than_p(&ann, p, mid) <= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    // x = 0 を弾く
    let lb = pass.max(1);

    let mut pass = p + 1;
    let mut fail = lb - 1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if count_no_more_than_p(&ann, p, mid) < k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let ub = pass;

    // println!("{} - {}", ub, lb);
    println!("{}", ub - lb);
}
