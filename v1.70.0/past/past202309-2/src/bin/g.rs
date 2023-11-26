use proconio::fastout;
use proconio::input;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a3n: [usize; n*3],
    }

    // FIXME: TLE するがわからず

    // 出力例 2 が枝刈りできない最悪ケース
    // 出力例 1 だと三角形を作る組 (A, B, C) と (A, B', C') を区別するかが読めない
    // 日本語を読む限りは, "ある三本の組が" であり区別する, 数えるでよいはず
    // すると HashSet に要素を入れ続ければだが 12! = 479_001_600 であり時間が怪しい
    // 時間 2s だしこれは想定解ではなさそう

    // 4^12 で 16_777_216 にしかならない, こっちか
    // でも 2650ms かかっている, なぜ？

    // 三角形を作りうる辺の組み合わせは 12C3 = 220 通り
    // 220C4 = 94,966,795 でありこれらを全探索でも TLE

    // あるいは 12C3 * 9C3 * 6C3 = 1320 * 504 * 120 = 79_833_600 でもだめ
    let mut hs = HashSet::new();
    let ptrn_max = n.pow(3 * n as u32);
    'outer_loop: for i in 0..ptrn_max {
        let mut v = vec![vec![]; n];
        let mut ii = i;
        for j in 0..n * 3 {
            v[ii % n].push(j);
            if v[ii % n].len() > 3 {
                continue 'outer_loop;
            }

            ii /= n;
        }

        for j in 0..n {
            v[j].sort_unstable();
        }
        v.sort_unstable();
        if hs.contains(&v) {
            continue;
        }

        let mut tri_pass = true;
        {
            for vv in &v {
                let a = a3n[vv[0]];
                let b = a3n[vv[1]];
                let c = a3n[vv[2]];
                if !(a + b > c && b + c > a && c + a > b) {
                    tri_pass = false;
                    break;
                }
            }
        }
        if !tri_pass {
            continue;
        }

        hs.insert(v);
    }

    println!("{}", hs.len());
}
