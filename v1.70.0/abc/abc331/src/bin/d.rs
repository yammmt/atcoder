// 商と余りが難しいと思うのだが editorial 見ると扱いは軽いのでそうではない？

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        pnn: [Chars; n],
        qq: [(usize, usize, usize, usize); q],
    }

    // (0, 0) と (i, j) を頂点とした範囲に含まれる黒マスを数える
    // 入力の左上頂点 A, B については -1 して扱う, これで範囲外に出るならその領域にマスはなし

    let mut cusumij = vec![vec![0; n]; n];
    for i in 0..n {
        let mut cusum_to_j = 0;
        for j in 0..n {
            if pnn[i][j] == 'B' {
                cusum_to_j += 1;
            }

            cusumij[i][j] = if i > 0 {
                cusumij[i - 1][j] + cusum_to_j
            } else {
                cusum_to_j
            };
        }
    }

    // pass (ex1)
    // for cs in &cusumij {
    //     println!("{:?}", cs);
    // }

    // 引数の型情報を抜くと cannot infer a type
    let cusum_w_r = |i: usize, j: usize| {
        let i_q = i / n;
        let j_q = j / n;
        let i_r = i % n;
        let j_r = j % n;

        // println!("div: {}, {}", i_q, j_q);
        // let a = cusumij[n - 1][n - 1] * i_q.min(j_q);
        let a = cusumij[n - 1][n - 1] * i_q * j_q;
        let b = cusumij[n - 1][j_r] * i_q;
        let c = cusumij[i_r][n - 1] * j_q;
        let d = cusumij[i_r][j_r];
        // println!("mod: {i_r} {j_r}");
        // println!("  {a} {b} {c} {d} = {}", a + b + c + d);
        a + b + c + d
    };

    // based on ex1
    // println!("{}", cusum_w_r(3, 0));
    // println!("{}", cusum_w_r(3, 1));
    // println!("{}", cusum_w_r(3, 2));
    // println!("{}", cusum_w_r(3, 3));
    // println!("{}", cusum_w_r(3, 4));
    // println!("{}", cusum_w_r(3, 5));
    // println!("{}", cusum_w_r(3, 6));
    // return;

    // マスが巨大
    for (a, b, c, d) in qq {
        let cd = cusum_w_r(c, d);
        let ad = if a == 0 { 0 } else { cusum_w_r(a - 1, d) };
        let cb = if b == 0 { 0 } else { cusum_w_r(c, b - 1) };
        let ab = if a == 0 || b == 0 {
            0
        } else {
            cusum_w_r(a - 1, b - 1)
        };
        // println!("{cd} + {ab} - {ad} - {cb}");
        println!("{}", cd + ab - ad - cb);
    }
}
