use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut sn: [(isize, isize); n],
        mut tn: [(isize, isize); n],
    }

    // S の任意の一点を T の任意の一点に動かし, 残りの点も同じように動かす, では O(N^2)
    // 操作をしても x/y 座標の片方はまったく変わらない
    // 対称の基準となる軸は (最大-最小)/2 じゃないかと思うがうまい説明ができない
    // なんにせよ操作不要と x/y 片方基準の平行移動を二度とで三度全一致を試せばよいだけ？
    // 0.5 が出てくる場合があるが, 0.5 なら誤差は出ないので無視してよいはず
    // あるいは入力全部倍にすれば解決する？
    // そもそも平行移動の式に x2 が出るから無視できるのか

    // 元から一致
    sn.sort_unstable();
    tn.sort_unstable();
    if sn == tn {
        println!("Yes");
        return;
    }

    let sy_min = sn.iter().map(|p| p.1).min().unwrap();
    let sy_max = sn.iter().map(|p| p.1).max().unwrap();
    let ty_min = tn.iter().map(|p| p.1).min().unwrap();
    let ty_max = tn.iter().map(|p| p.1).max().unwrap();

    let sx_min = sn.iter().map(|p| p.0).min().unwrap();
    let sx_max = sn.iter().map(|p| p.0).max().unwrap();
    let tx_min = tn.iter().map(|p| p.0).min().unwrap();
    let tx_max = tn.iter().map(|p| p.0).max().unwrap();

    // x 軸に平行な線に対称となる移動
    let y_mid_x2 = sy_max.max(ty_max) + sy_min.min(ty_min);
    let mut s_x = Vec::with_capacity(n);
    for s in &sn {
        s_x.push((s.0, y_mid_x2 - s.1));
    }
    s_x.sort_unstable();
    if s_x == tn {
        println!("Yes");
        return;
    }

    // y 軸に平行な線に対称となる移動
    let x_mid_x2 = sx_max.max(tx_max) + sx_min.min(tx_min);
    let mut s_y = Vec::with_capacity(n);
    for s in &sn {
        s_y.push((x_mid_x2 - s.0, s.1));
    }
    s_y.sort_unstable();
    if s_y == tn {
        println!("Yes");
        return;
    }

    println!("No");
}
