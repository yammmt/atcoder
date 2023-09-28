use proconio::input;
use proconio::marker::Chars;

fn cut_pass(n: usize, m: usize, k: usize, cur_map: &[Vec<char>], size: usize) -> bool {
    if size > n || size > m {
        return false;
    }

    for i in 0..n - size + 1 {
        for j in 0..m - size + 1 {
            let mut counts = vec![0; 10];
            // println!("from: {} {}", i, j);

            for ii in 0..size {
                for jj in 0..size {
                    counts[(cur_map[i + ii][j + jj] as u8 - b'0') as usize] += 1;
                }
            }

            // 自分以外を全部置き換える
            if counts.iter().any(|&c| size * size - c <= k) {
                return true;
            }
        }
    }

    false
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        snm: [Chars; n],
    }

    // 数字の替え方を総当りすることは無理
    // 正方形サイズを固定して達成可能か判定する二分探索

    // 二分探索の上限判定を先に取っておくとどうしてか WA
    // if cut_pass(n, m, k, &snm, n.min(m)) {
    //     println!("{}", n.min(m));
    //     return;
    // }

    let mut pass = 1;
    // let mut fail = n.min(m);
    let mut fail = n.max(m) + 1;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        // println!("{} {}, {}", pass, fail, mid);
        if cut_pass(n, m, k, &snm, mid) {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
