// :fu: :fu: 添字

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i64,
        mut xvn: [(i64, i64); n],
    }

    let mut ans = 0;

    let mut xvrn = xvn.clone();
    xvrn.reverse();
    let mut ccw = Vec::with_capacity(n + 1);
    // 距離, カロリー
    ccw.push((0, 0));
    let mut cur = 0;
    // 反時計回りに i 個取る
    for (i, xv) in xvrn.iter().enumerate() {
        cur += xv.1;
        ccw.push(
            if cur - (c - xv.0) <= ccw[i].1 - ccw[i].0 {
                // 直前の値で止めるべき
                ccw[i]
            } else {
                // 食すべき
                (c - xv.0, cur)
            }
        );
    }
    // println!("{:?}", ccw);

    // 時計回りに i 個取る
    let mut cw = Vec::with_capacity(n + 1);
    cw.push((0, 0));
    let mut cur = 0;
    for (i, xv) in xvn.iter().enumerate() {
        cur += xv.1;
        cw.push(
            if cur - (xv.0) <= cw[i].1 - cw[i].0 {
                cw[i]
            } else {
                (xv.0, cur)
            }
        );
    }
    // println!("{:?}", cw):

    // 時計回りに i 個, 反時計回りに n - i 個を上限として取る場合の最適値
    // "上限として" が抜けると引くべき移動距離がバグる
    for i in 0..n + 1 {
        ans = ans.max(cw[i].1 + ccw[n-i].1 - cw[i].0 - ccw[n-i].0 - cw[i].0.min(ccw[n-i].0));
    }

    println!("{}", ans);
}
