// 80min 1WA (70min)
// WA: "2 0 0 8" で "-1" になっていなかった
// 式を立てるのに苦労

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n + 1],
    }
    // 後ろの式で i = 0 時を無視しているため
    // 制約より n = 0 かつ an = 0 は存在しない
    if n == 0 && an[0] != 1 {
        println!("-1");
        return;
    }

    // 増やせるだけ増やして以後一頂点から一頂点に線を張っていく
    // 矛盾したらアウト
    let mut leaf_left = an.iter().sum::<i64>();
    let mut vertexes = vec![0i64; n + 1];
    vertexes[0] = 1;
    for i in 1..n + 1 {
        vertexes[i] = (2 * (vertexes[i - 1] - an[i - 1])).min(leaf_left);
        if vertexes[i] < an[i] || (i == n && vertexes[i] != an[i]) {
            println!("-1");
            return;
        }
        leaf_left -= an[i];
    }

    println!(
        "{}",
        if vertexes.iter().any(|&a| a <= 0) {
            -1
        } else {
            vertexes.iter().sum::<i64>()
        }
    );
}
