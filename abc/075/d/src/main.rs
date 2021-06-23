// :fu: 21-06 もたつく

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xyn: [(i64, i64); n],
    }

    // 座標幅が狭ければ片方の軸の始点終点を決め打ちできたが今回は不可
    // 点数が小さければ含む点を全列挙すれば良いが今回は不可 (50C25 > 1e14)
    // 左上の点を固定して一つ右の点/一つ下の点を足していくと二通りの分岐が最大 50 回発生して TLE
    //  あるいは重複分を除くと間に合う？でも明らかな重実装
    // 座圧して累積和は面積を出せない？
    // 全点を選んだ状態から始めると一点ずつ消していけば良いが

    // 50^4 = 6,250,000 で全点舐めても x 50
    let mut ans = std::i64::MAX;
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                for d in 0..n {
                    let x_range = (
                        xyn[a].0.min(xyn[b].0.min(xyn[c].0.min(xyn[d].0))),
                        xyn[a].0.max(xyn[b].0.max(xyn[c].0.max(xyn[d].0))),
                    );
                    let y_range = (
                        xyn[a].1.min(xyn[b].1.min(xyn[c].1.min(xyn[d].1))),
                        xyn[a].1.max(xyn[b].1.max(xyn[c].1.max(xyn[d].1))),
                    );

                    let included = xyn
                        .iter()
                        .filter(|xy| {
                            x_range.0 <= xy.0
                                && xy.0 <= x_range.1
                                && y_range.0 <= xy.1
                                && xy.1 <= y_range.1
                        })
                        .count();
                    if included >= k {
                        ans = ans.min(((x_range.1 - x_range.0) * (y_range.1 - y_range.0)).abs());
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
