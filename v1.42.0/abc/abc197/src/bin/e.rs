use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xcn: [(i64, u64); n],
    }
    // 色 -> 座標でソートすれば
    xcn.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    // (今居る座標, ここまでの合計距離)
    let mut to_biggest = (0, 0);
    let mut to_shortest = (0, 0);
    let mut x_min = std::i64::MAX / 2;
    let mut x_max = std::i64::MIN / 2;
    let mut x_i = 0;
    while x_i < n {
        if x_i > 0 && xcn[x_i].1 != xcn[x_i - 1].1 {
            let mut to_biggest_new = (0, std::i64::MAX / 2);
            // 旧大 -> 新小 -> 新大
            let next_d_from_b = to_biggest.1 + (to_biggest.0 - x_min).abs() + (x_max - x_min).abs();
            // 旧小 -> 新小 -> 新大
            let next_d_from_s = to_shortest.1 + (to_shortest.0 - x_min).abs() + (x_max - x_min).abs();
            to_biggest_new = (x_max, next_d_from_b.min(next_d_from_s));

            let mut to_shortest_new = (0, std::i64::MAX / 2);
            // 旧大 -> 新大 -> 新小
            let next_d_from_b = to_biggest.1 + (to_biggest.0 - x_max).abs() + (x_max - x_min).abs();
            // 旧小 -> 新大 -> 新小
            let next_d_from_s = to_shortest.1 + (to_shortest.0 - x_max).abs() + (x_max - x_min).abs();
            to_shortest_new = (x_min, next_d_from_b.min(next_d_from_s));

            to_biggest = to_biggest_new;
            to_shortest = to_shortest_new;

            x_min = std::i64::MAX / 2;
            x_max = std::i64::MIN / 2;
        }

        x_min = x_min.min(xcn[x_i].0);
        x_max = x_max.max(xcn[x_i].0);
        x_i += 1;
    }

    // ラスト一回分はループ外
    let mut to_biggest_new = (0, std::i64::MAX / 2);
    // 旧大 -> 新小 -> 新大
    let next_d_from_b = to_biggest.1 + (to_biggest.0 - x_min).abs() + (x_max - x_min).abs();
    // 旧小 -> 新小 -> 新大
    let next_d_from_s = to_shortest.1 + (to_shortest.0 - x_min).abs() + (x_max - x_min).abs();
    to_biggest_new = (x_max, next_d_from_b.min(next_d_from_s));

    let mut to_shortest_new = (0, std::i64::MAX / 2);
    // 旧大 -> 新大 -> 新小
    let next_d_from_b = to_biggest.1 + (to_biggest.0 - x_max).abs() + (x_max - x_min).abs();
    // 旧小 -> 新大 -> 新小
    let next_d_from_s = to_shortest.1 + (to_shortest.0 - x_max).abs() + (x_max - x_min).abs();
    to_shortest_new = (x_min, next_d_from_b.min(next_d_from_s));

    to_biggest = to_biggest_new;
    to_shortest = to_shortest_new;

    println!(
        "{}",
        (to_biggest.0.abs() + to_biggest.1).min(to_shortest.0.abs() + to_shortest.1)
    );
}
