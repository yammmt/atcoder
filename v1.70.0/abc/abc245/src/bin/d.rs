// とてもつらい

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [i64; n + 1],
        cnm: [i64; n + m + 1],
    }

    let mut bm = vec![0; m + 1];
    for i in (0..=m).rev() {
        // an[n] で割る行為と対応が取れる範囲
        // println!("i: {i}");
        let c_idx = i + n;
        // println!("  c_idx: {c_idx}");
        let mut rhv = 0;
        // a, b の添字 i, j の和が c の添字に等しく, かつ j が今求めようとしている値では
        // ないこと
        // j 始点: bm[i] より大きい添字の値はここまでに求まっているので使える
        // 上限が m: bm の最大要素に到達
        // 上限が c_idx: c の次数が m より小さいため打ち切る
        for j in i + 1..=m.min(c_idx) {
            let k = c_idx - j;
            // println!("  j: {j}, k: {k}");
            rhv += bm[j] * an[k];
        }

        // println!("  rhv: {rhv}");
        bm[i] = (cnm[c_idx] - rhv) / an[n];
        // println!("  bm[i]: {}", bm[i]);
    }

    for (i, b) in bm.iter().enumerate() {
        if i == m {
            println!("{b}");
        } else {
            print!("{b} ");
        }
    }
}
