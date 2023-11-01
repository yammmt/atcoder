use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        h: isize,
        w: isize,
        n: usize,
        abn: [(isize, isize); n],
    }

    // マスを一つ塗ったとして, そのマスを含む 3x3 の正方形は最大で 9 個
    // (そのマスが 9 マスのどこにあたるか)
    // 一つ一つの入力に対し, 正方形の (左上, 右下) の座標を管理したとして
    // 最大でその数は N * 9 で 10^6 程度
    // これなら十分に計算できそう

    let mut p_and_blk = HashMap::new();
    for ab in abn {
        let a = ab.0;
        let b = ab.1;

        for i in -2..=0 {
            let a_lt = a + i;
            let a_rb = a_lt + 2;
            if a_lt <= 0 || a_rb > h {
                continue;
            }

            for j in -2..=0 {
                let b_lt = b + j;
                let b_rb = b_lt + 2;
                if b_lt <= 0 || b_rb > w {
                    continue;
                }

                // ((始点, 終点))
                let cnt = p_and_blk.entry(((a_lt, b_lt), (a_rb, b_rb))).or_insert(0);
                *cnt += 1;
            }
        }
    }

    let mut ans = vec![0; 10];
    for i in 1..=9 {
        let cur = p_and_blk.iter().filter(|(k, v)| **v == i).count();
        ans[i] = cur as isize;
    }
    // thank you サンプル 3
    ans[0] = (h - 2) * (w - 2) - ans.iter().sum::<isize>();

    for a in ans {
        println!("{a}");
    }
}
